// ssh_agent_client_rs_git_bash/src/lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # ssh_agent_client_rs_git_bash
//!
//! **Add git-bash ssh-agent implementation for nresare/ssh-agent-client-rs**  
//! ***version: 0.0.15 date: 2025-03-30 author: [Bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!
//!  [![crates.io](https://img.shields.io/crates/v/ssh_agent_client_rs_git_bash.svg)](https://crates.io/crates/ssh_agent_client_rs_git_bash)
//!  [![Documentation](https://docs.rs/ssh_agent_client_rs_git_bash/badge.svg)](https://docs.rs/ssh_agent_client_rs_git_bash/)
//!  ![License](https://img.shields.io/badge/license-MIT-blue.svg)
//!  ![Rust](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash/workflows/rust_fmt_auto_build_test/badge.svg)
//!  ![ssh_agent_client_rs_git_bash](https://bestia.dev/webpage_hit_counter/get_svg_image/928692335.svg)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-151-green.svg)](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-119-blue.svg)](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-14-purple.svg)](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-99-orange.svg)](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash/)
//!
//! Hashtags: #maintained #ready-for-use #rustlang  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
//!
//! ## Description
//!
//! This is an extension for the crate [nresare/ssh-agent-client-rs](https://github.com/nresare/ssh-agent-client-rs).  
//! It adds the implementation for windows git-bash.  
//! The original dependency is re-exported.  
//! Instead of adding dependency to `ssh-agent-client-rs`, add `ssh_agent_client_rs_git_bash`.  
//! Instead of `Client::connect()`, use the method `Client::connect_to_git_bash_or_linux()` of the new trait:
//!
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! ssh_agent_client_rs_git_bash = {git="https://github.com/bestia-dev/ssh_agent_client_rs_git_bash.git"}
//! ```
//!
//! ```rust ignore
//! use ssh_agent_client_rs_git_bash::Client;
//! // import trait to scope
//! use ssh_agent_client_rs_git_bash::GitBash;
//! let mut client = Client::connect_to_git_bash_or_linux(&path_ssh_auth_sock)
//!     .expect("Cannot connect to ssh-agent.");
//! // then normal code with Client
//! client.list-identities().unwrap;
//! ```
//!
//! ## Windows git-bash
//!
//! The implementation of ssh-agent in git-bash works over Tcp socket and is supported by this client.  
//! The rust target is x86_64-pc-windows-gnu and can be cross-compiled from Linux to Windows.  
//! Windows git-bash environment has also other names: cygwin, msys2, mingW64, git-for-windows, ...  
//!
//! Windows has other not compatible ssh-agent implementations that are NOT supported by this client.  
//!
//! * Microsoft ssh works over named pipes.
//! * After 2019 Microsoft introduced Unix Sockets.
//! * Old and obsolete msys or msysGit
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

mod error;
#[cfg(target_family = "windows")]
use error::Error;
use error::Result;

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
    fn connect_to_git_bash_or_linux(path: &std::path::Path) -> Result<Client>;
}

/// Add methods to struct implementation
impl GitBash for Client {
    /// Constructs a Client connected to a tcp socket for 'windows git-bash' or Linux Domain Socket.
    ///
    /// Conditional compiling depends on the target family.
    #[cfg(target_family = "windows")]
    fn connect_to_git_bash_or_linux(path: &std::path::Path) -> Result<Client> {
        let (tcp_address, key_guid) = read_and_parse_fake_socket_file(path)?;
        let mut tcp_stream = std::net::TcpStream::connect(&tcp_address)?;
        do_secret_handshake_with_remote_end(&key_guid, &mut tcp_stream)?;
        Ok(Client::with_read_write(Box::new(tcp_stream)))
    }

    /// Constructs a Client connected to a tcp socket for 'windows git-bash' or Linux Domain Socket.
    ///
    /// Conditional compiling depends on the target family.
    #[cfg(target_family = "unix")]
    fn connect_to_git_bash_or_linux(path: &std::path::Path) -> Result<Client> {
        Ok(Client::connect(path)?)
    }
}

/// Read and parse fake socket metadata.
///
/// ssh-agent exports the env variable SSH_AUTH_SOCK. This is normally the paths to the Unix Socket.
/// In 'windows git-bash' the fake unix domain socket path is just a normal file
/// that contains some data for the tcp connection.
/// example: `!<socket >49722 s 09B97624-72E2CDC5-38596B86-E9F0B690\0`
#[cfg(target_family = "windows")]
fn read_and_parse_fake_socket_file(path: &std::path::Path) -> Result<(String, String)> {
    let conn_string = std::fs::read_to_string(path)?;
    let (tcp_address, key_guid) = parse_fake_socket_metadata(&conn_string)?;
    Ok((tcp_address, key_guid))
}

/// Secret handshake only for ssh-agent in git-bash.
#[cfg(target_family = "windows")]
fn do_secret_handshake_with_remote_end(key_guid: &str, tcp_stream: &mut std::net::TcpStream) -> Result<()> {
    use std::io::{Read, Write};
    let b1 = parse_guid_and_change_byte_order(key_guid)?;
    let _amount = tcp_stream.write(&b1)?;
    let mut b2: [u8; 16] = [0; 16];
    let _amount = tcp_stream.read(&mut b2)?;
    let pid_uid_gid = prepare_pid_uid_gid()?;
    let _amount = tcp_stream.write(&pid_uid_gid)?;
    let mut b3: [u8; 16] = [0; 16];
    let _amount = tcp_stream.read(&mut b3)?;
    Ok(())
}

/// Parse fake socket metadata.
///
/// example: `!<socket >49722 s 09B97624-72E2CDC5-38596B86-E9F0B690\0`
#[cfg(target_family = "windows")]
fn parse_fake_socket_metadata(conn_string: &str) -> Result<(String, String)> {
    let conn_string = conn_string.trim_start_matches("!<socket >").trim_end_matches("\0");
    let mut split_iter = conn_string.split_whitespace();
    let tcp_port = split_iter
        .next()
        .ok_or_else(|| Error::GitBashErrorMessage("Bad format in ssh agent connection file.".to_string()))?;
    let is_cygwin = split_iter
        .next()
        .ok_or_else(|| Error::GitBashErrorMessage("Bad format in ssh agent connection file.".to_string()))?;
    let key_guid = split_iter
        .next()
        .ok_or_else(|| Error::GitBashErrorMessage("Bad format in ssh agent connection file.".to_string()))?;
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

/// The handshake needs these 3 values: pid uid gid.
///
/// The bytes order are reversed.
#[cfg(target_family = "windows")]
fn prepare_pid_uid_gid() -> Result<[u8; 12]> {
    let pid: u32 = std::process::id();
    let uid = get_uid()?;
    // for cygwin's AF_UNIX -> AF_INET, pid = gid"
    let gid = pid;

    let pid_uid_gid = order_bytes_pid_uid_gid(pid, uid, gid).unwrap();
    Ok(pid_uid_gid)
}

/// Get uid from the bash command 'ps'.
#[cfg(target_family = "windows")]
fn get_uid() -> Result<u32> {
    let vec_byte_out = std::process::Command::new(r#"C:\Program Files\Git\usr\bin\bash.exe"#)
        .arg("-c")
        .arg("ps")
        .output()?
        .stdout;
    let string_output = String::from_utf8_lossy(&vec_byte_out);
    let uid = parse_uid(string_output)?;
    Ok(uid)
}

/// Change order of bytes for pid, uid and gid.
///
/// Every u32 is converted to LittleEndian.
#[cfg(target_family = "windows")]
fn order_bytes_pid_uid_gid(pid: u32, uid: u32, gid: u32) -> Result<[u8; 12]> {
    let mut pid_uid_gid: [u8; 12] = [0; 12];
    pid_uid_gid[0..4].swap_with_slice(&mut pid.to_le_bytes());
    pid_uid_gid[4..8].swap_with_slice(&mut uid.to_le_bytes());
    pid_uid_gid[8..12].swap_with_slice(&mut gid.to_le_bytes());
    Ok(pid_uid_gid)
}

/// Parse uid from 'ps' bash command.
#[cfg(target_family = "windows")]
fn parse_uid(string_output: std::borrow::Cow<'_, str>) -> Result<u32> {
    // The output is like this:
    //       PID    PPID    PGID     WINPID   TTY         UID    STIME COMMAND
    //      1344       1    1344      15352  ?         197610 13:36:43 /usr/bin/ssh-agent
    //      2542       1    2542      21776  cons1     197610 19:09:45 /usr/bin/bash
    // The UID is equal for all rows. We will use the second row.
    let mut lines = string_output.lines();
    let _line_0 = lines
        .next()
        .ok_or_else(|| Error::GitBashErrorMessage("Command 'ps' did not return correct list.".to_string()))?;
    let line_1 = lines
        .next()
        .ok_or_else(|| Error::GitBashErrorMessage("Command 'ps' did not return correct list.".to_string()))?;
    let mut columns = line_1.split_ascii_whitespace();
    // The 5th column is the UID.
    let uid: u32 = columns
        .nth(5)
        .ok_or_else(|| Error::GitBashErrorMessage("Command 'ps' did not return correct list.".to_string()))?
        .parse()
        .map_err(|_| Error::GitBashErrorMessage("Format of 'bash.exe -c ps' is incorrect.".to_string()))?;
    Ok(uid)
}

/// Parse guid and change byte order.
///
/// Original guid looks like this: 01020304-05060708-090A0B0C-0D0E0F10.
/// Two hexadecimals digits form one u8 byte. There are 4 groups with 4 u8 bytes.
/// Eight hexadecimal digits form one u32 byte. That is one group.
#[cfg(target_family = "windows")]
fn parse_guid_and_change_byte_order(key_guid: &str) -> Result<[u8; 16]> {
    let group0 = u32::from_str_radix(&key_guid[0..8], 16)
        .map_err(|_| Error::GitBashErrorMessage("Guid in SSH_AUTH_SOCK is incorrect.".to_string()))?;
    let group1 = u32::from_str_radix(&key_guid[9..17], 16)
        .map_err(|_| Error::GitBashErrorMessage("Guid in SSH_AUTH_SOCK is incorrect.".to_string()))?;
    let group2 = u32::from_str_radix(&key_guid[18..26], 16)
        .map_err(|_| Error::GitBashErrorMessage("Guid in SSH_AUTH_SOCK is incorrect.".to_string()))?;
    let group3 = u32::from_str_radix(&key_guid[27..35], 16)
        .map_err(|_| Error::GitBashErrorMessage("Guid in SSH_AUTH_SOCK is incorrect.".to_string()))?;
    // The secret handshake converts the u32 into LittleEndian.
    // Nobody knows why is that needed, but it is the protocol.
    let mut b1: [u8; 16] = [0; 16];
    b1[0..4].swap_with_slice(&mut group0.to_le_bytes());
    b1[4..8].swap_with_slice(&mut group1.to_le_bytes());
    b1[8..12].swap_with_slice(&mut group2.to_le_bytes());
    b1[12..16].swap_with_slice(&mut group3.to_le_bytes());
    Ok(b1)
}

#[cfg(test)]
mod tests {
    #[cfg(target_family = "windows")]
    use super::*;

    #[test]
    #[cfg(target_family = "windows")]
    fn test_parse_fake_socket_metadata() {
        let conn_string = "!<socket >49722 s 09B97624-72E2CDC5-38596B86-E9F0B690\0";
        let (tcp_address, key_guid) = parse_fake_socket_metadata(conn_string).unwrap();
        assert_eq!(tcp_address, "localhost:49722");
        assert_eq!(key_guid, "09B97624-72E2CDC5-38596B86-E9F0B690");
    }

    #[test]
    #[cfg(target_family = "windows")]
    fn test_parse_uid() {
        let string_output = r#"       PID    PPID    PGID     WINPID   TTY         UID    STIME COMMAND
      1344       1    1344      15352  ?         197610 13:36:43 /usr/bin/ssh-agent
      2542       1    2542      21776  cons1     197610 19:09:45 /usr/bin/bash
"#;
        let uid = parse_uid(string_output.into()).unwrap();
        assert_eq!(uid, 197610);
    }

    #[test]
    #[cfg(target_family = "windows")]
    fn test_parse_guid_and_change_byte_order() {
        let guid = "09B97624-72E2CDC5-38596B86-E9F0B690";
        let ordered_guid = parse_guid_and_change_byte_order(guid).unwrap();
        let compare_with: [u8; 16] = [36, 118, 185, 9, 197, 205, 226, 114, 134, 107, 89, 56, 144, 182, 240, 233];
        assert_eq!(ordered_guid, compare_with);
    }

    #[test]
    #[cfg(target_family = "windows")]
    fn test_order_bytes_pid_uid_gid() {
        let pid_uid_gid = order_bytes_pid_uid_gid(1, 2, 3).unwrap();
        let compare_with: [u8; 12] = [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0];
        assert_eq!(pid_uid_gid, compare_with);
    }
}
