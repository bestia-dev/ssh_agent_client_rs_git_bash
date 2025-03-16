<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

# ssh_agent_client_rs_git_bash

[//]: # (auto_cargo_toml_to_md start)

**Add git-bash ssh-agent implementation for nresare/ssh-agent-client-rs**  
***version: 0.0.11 date: 2025-03-16 author: [Bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash)***

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
 ![rustlang](https://img.shields.io/badge/rustlang-orange)

[//]: # (auto_cargo_toml_to_md end)

 [![crates.io](https://img.shields.io/crates/v/ssh_agent_client_rs_git_bash.svg)](https://crates.io/crates/ssh_agent_client_rs_git_bash)
 [![Documentation](https://docs.rs/ssh_agent_client_rs_git_bash/badge.svg)](https://docs.rs/ssh_agent_client_rs_git_bash/)
 ![License](https://img.shields.io/badge/license-MIT-blue.svg)
 ![Rust](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash/workflows/rust_fmt_auto_build_test/badge.svg)
 ![ssh_agent_client_rs_git_bash](https://bestia.dev/webpage_hit_counter/get_svg_image/928692335.svg)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-164-green.svg)](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-113-blue.svg)](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-14-purple.svg)](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-130-orange.svg)](https://github.com/bestia-dev/ssh_agent_client_rs_git_bash/)

[//]: # (auto_lines_of_code end)

Hashtags: #maintained #ready-for-use #rustlang  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  

## Description

This is an extension for the crate [nresare/ssh-agent-client-rs](https://github.com/nresare/ssh-agent-client-rs).  
It adds the implementation for windows git-bash.  
The original dependency is re-exported.  
Instead of adding dependency to `ssh-agent-client-rs`, add `ssh_agent_client_rs_git_bash`.  
Instead of `Client::connect()`, use the method `Client::connect_to_git_bash_or_linux()` of the new trait:

```toml
# Cargo.toml
[dependencies]
ssh_agent_client_rs_git_bash = {git="https://github.com/bestia-dev/ssh_agent_client_rs_git_bash.git"}
```

```rust ignore
use ssh_agent_client_rs_git_bash::Client;
// import trait to scope
use ssh_agent_client_rs_git_bash::GitBash;
let mut client = Client::connect_to_git_bash_or_linux(&path_ssh_auth_sock)
    .expect("Cannot connect to ssh-agent.");
// then normal code with Client
client.list-identities().unwrap;
```

## Windows git-bash

The implementation of ssh-agent in git-bash works over Tcp socket and is supported by this client.  
The rust target is x86_64-pc-windows-gnu and can be cross-compiled from Linux to Windows.  
Windows git-bash environment has also other names: cygwin, msys2, mingW64, git-for-windows, ...  

Windows has other not compatible ssh-agent implementations that are NOT supported by this client.  

* Microsoft ssh works over named pipes.
* After 2019 Microsoft introduced Unix Sockets.
* Old and obsolete msys or msysGit

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
