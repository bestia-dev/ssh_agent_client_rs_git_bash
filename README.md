# ssh_agent_client_rs_git_bash

This is an extension for the crate [ssh-agent-client-rs](https://github.com/nresare/ssh-agent-client-rs).  
It adds the implementation for windows git-bash.

## Windows git-bash

The implementation of ssh-agent in git-bash works over Tcp socket and is supported by this client.  
The rust target is x86_64-pc-windows-gnu and can be cross-compiled from Linux to Windows.  
Windows git-bash environment has also other names: cygwin, msys2, mingW64, git-for-windows, ...  

Windows has other not compatible ssh-agent implementations that are NOT supported by this client.  

* Microsoft ssh works over named pipes.
* After 2019 Microsoft introduced Unix Sockets.
* Old and obsolete msys or msysGit
