# Catsploit Lib
The library for Catsploit, contains all the functional code for carrying out exploits.  

- `core` - The core functionality such as the `Exploit` trait, module `Opt` struct, TCP handlers for revshell `GenericTcpHandler`. Contains the glue code that all modules rely on basically.
- `module` - Indivdual Catsploit modules are defined here, such as the `Vsftpd234Backdoor` exploit, the `NcMkfifoReverseTcp` payload. Modules must implement a module trait, such as `Exploit` or `Payload`, and are then added to the `index.rs` which is were applications such as `catsploit` CLI will hook into.
- `util` - For miscellaneous functionality such as generating random alphanumerics etc.
