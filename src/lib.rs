// ssh_agent_client_rs_git_bash/src/lib.rs

mod error;
use error::{Error, Result};

use std::io::{Read, Write};

/// Re-export ssh-agent-client-rs Client.
pub use ssh_agent_client_rs::Client;

/// Trait adds new methods to the struct implementation.
///
/// On Windows, git-for-windows, git-bash, cygwin, msysgit, msys2 and mingW64 provide functionality similar to a Linux distribution.  
/// Linux uses UnixStream, but Windows before 2019 didn't have UDS 'Unix Domain Socket'.  
/// Windows "git-bash" needed a different way for "ssh-add" (client) and "ssh-agent" (server) for inter process communication.  
/// They invented a special protocol and use the Tcp Socket instead of Unix Socket.  
/// <https://stackoverflow.com/questions/23086038/what-mechanism-is-used-by-msys-cygwin-to-emulate-unix-domain-sockets>
/// <https://github.com/abourget/secrets-bridge/blob/master/pkg/agentfwd/agentconn_windows.go>
pub trait GitBash {
    fn connect_to_git_bash_ssh_agent(path: &std::path::Path) -> Result<Client>;
}

/// Add methods to struct implementation
impl GitBash for Client {
    /// Constructs a Client connected to a tcp socket for 'windows git-bash'.
    #[cfg(target_family = "windows")]
    fn connect_to_git_bash_ssh_agent(path: &std::path::Path) -> Result<Client> {
        let (tcp_address, key_guid) = parse_fake_socket_metadata(path)?;
        let mut tcp_stream = std::net::TcpStream::connect(&tcp_address)?;
        do_secret_handshake_with_remote_end(&key_guid, &mut tcp_stream)?;
        Ok(Client::with_read_write(Box::new(tcp_stream)))
    }
}

/// Secret handshake only for ssh-agent in git-bash.
fn do_secret_handshake_with_remote_end(
    key_guid: &str,
    tcp_stream: &mut std::net::TcpStream,
) -> Result<()> {
    let b1 = parse_guid_and_change_byte_order(key_guid)?;
    let _amount = tcp_stream.write(&b1)?;
    let mut b2: [u8; 16] = [0; 16];
    let _amount = tcp_stream.read(&mut b2)?;
    let pid_uid_gid = prepare_pid_gid_uid()?;
    let _amount = tcp_stream.write(&pid_uid_gid)?;
    let mut b3: [u8; 16] = [0; 16];
    let _amount = tcp_stream.read(&mut b3)?;
    Ok(())
}

/// Parse fake socket metadata.
///
/// ssh-agent exports the env variable SSH_AUTH_SOCK. This is normally the paths to the Unix Socket.
/// In 'windows git-bash' the fake unix domain socket path is just a normal file
/// that contains some data for the tcp connection.
fn parse_fake_socket_metadata(path: &std::path::Path) -> Result<(String, String)> {
    let conn_string = std::fs::read_to_string(path)?;
    // example: !<socket >49722 s 09B97624-72E2CDC5-38596B86-E9F0B690\0
    let conn_string = conn_string
        .trim_start_matches("!<socket >")
        .trim_end_matches("\0");
    let mut split_iter = conn_string.split_whitespace();
    let tcp_port = split_iter.next().ok_or_else(|| {
        Error::GitBashErrorMessage("Bad format in ssh agent connection file.".to_string())
    })?;
    let is_cygwin = split_iter.next().ok_or_else(|| {
        Error::GitBashErrorMessage("Bad format in ssh agent connection file.".to_string())
    })?;
    let key_guid = split_iter.next().ok_or_else(|| {
        Error::GitBashErrorMessage("Bad format in ssh agent connection file.".to_string())
    })?;
    // The character 's' defines the newer version of MSys2 or cygwin or mingw64.
    // Only this ssh-agent implementation is supported. The older are not supported.
    if is_cygwin != "s" {
        return Err(Error::GitBashErrorMessage(
            "Old version of MSysGit ssh-agent implementation is not supported.".to_string(),
        ));
    }
    let tcp_address = format!("localhost:{}", tcp_port);
    Ok((tcp_address, key_guid.to_string()))
}

/// The handshake needs these 3 values: pid gid uid.
///
/// The bytes order are reversed.
fn prepare_pid_gid_uid() -> Result<[u8; 12]> {
    let mut pid_uid_gid: [u8; 12] = [0; 12];
    let pid: u32 = std::process::id();
    // convert to LittleEndian
    pid_uid_gid[0..4].swap_with_slice(&mut pid.to_le_bytes());
    let vec_byte_out = std::process::Command::new(r#"C:\Program Files\Git\usr\bin\bash.exe"#)
        .arg("-c")
        .arg("ps")
        .output()?
        .stdout;
    let string_output = String::from_utf8_lossy(&vec_byte_out);
    // The output is like this:
    //       PID    PPID    PGID     WINPID   TTY         UID    STIME COMMAND
    //      1344       1    1344      15352  ?         197610 13:36:43 /usr/bin/ssh-agent
    //      2542       1    2542      21776  cons1     197610 19:09:45 /usr/bin/bash
    // The UID is equal for all rows. We will use the second row.
    let mut lines = string_output.lines();
    let _line_0 = lines.next().ok_or_else(|| {
        Error::GitBashErrorMessage("Command 'ps' did not return correct list.".to_string())
    })?;
    let line_1 = lines.next().ok_or_else(|| {
        Error::GitBashErrorMessage("Command 'ps' did not return correct list.".to_string())
    })?;
    // The 5th column is the UID.
    let mut columns = line_1.split_ascii_whitespace();
    let uid: u32 = columns
        .nth(5)
        .ok_or_else(|| {
            Error::GitBashErrorMessage("Command 'ps' did not return correct list.".to_string())
        })?
        .parse()
        .map_err(|_| {
            Error::GitBashErrorMessage("Format of 'bash.exe -c ps' is incorrect.".to_string())
        })?;
    // convert to LittleEndian
    pid_uid_gid[4..8].swap_with_slice(&mut uid.to_le_bytes());
    // for cygwin's AF_UNIX -> AF_INET, pid = gid"
    let gid = pid;
    // convert to LittleEndian
    pid_uid_gid[8..12].swap_with_slice(&mut gid.to_le_bytes());
    Ok(pid_uid_gid)
}

/// Parse guid and change byte order.
///
/// Original guid looks like this: 01020304-05060708-090A0B0C-0D0E0F10.
/// Two hexadecimals digits form one u8 byte. There are 4 groups with 4 u8 bytes.
/// Eight hexadecimal digits form one u32 byte. That is one group.
fn parse_guid_and_change_byte_order(key_guid: &str) -> Result<[u8; 16]> {
    let group0 = u32::from_str_radix(&key_guid[0..8], 16).map_err(|_| {
        Error::GitBashErrorMessage("Guid in SSH_AUTH_SOCK is incorrect.".to_string())
    })?;
    let group1 = u32::from_str_radix(&key_guid[9..17], 16).map_err(|_| {
        Error::GitBashErrorMessage("Guid in SSH_AUTH_SOCK is incorrect.".to_string())
    })?;
    let group2 = u32::from_str_radix(&key_guid[18..26], 16).map_err(|_| {
        Error::GitBashErrorMessage("Guid in SSH_AUTH_SOCK is incorrect.".to_string())
    })?;
    let group3 = u32::from_str_radix(&key_guid[27..35], 16).map_err(|_| {
        Error::GitBashErrorMessage("Guid in SSH_AUTH_SOCK is incorrect.".to_string())
    })?;
    // The secret handshake converts the u32 into LittleEndian.
    // Nobody knows why is that needed, but it is the protocol.
    let mut b1: [u8; 16] = [0; 16];
    b1[0..4].swap_with_slice(&mut group0.to_le_bytes());
    b1[4..8].swap_with_slice(&mut group1.to_le_bytes());
    b1[8..12].swap_with_slice(&mut group2.to_le_bytes());
    b1[12..16].swap_with_slice(&mut group3.to_le_bytes());
    Ok(b1)
}
