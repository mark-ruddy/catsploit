# Catsploit
Catsploit is an open-source modern exploitation framework.  

Catsploit is currently in early development and the project is aiming to attract contributors who are interested in building the next generation exploitation framework in Rust. The project is intended to stay 100% open-source with no premium version, and is licensed under GPLv3.

## Install
To install as a crate: `cargo install catsploit`

To build from source:

```
git clone https://github.com/tirax-lab/catsploit
cd catsploit/catsploit
cargo build --release
sudo cp ./target/release/catsploit /usr/local/bin
```

## Example Usage - Exploiting the VSFTPD v2.3.4 Backdoor
In this exploitation a virtual machine with Metasploitable2 is running at `172.16.187.128`, which has a vulnerable `VSFTPD` server running:

- The default reverse shell `nc_mkfifo_reverse_tcp` payload has its `LHOST` set to `172.16.1.1` which is where VMware routes back to the host machine
- The VSFTPD exploit has its `RHOST` set to `172.16.187.128` and the default `RPORT` is `21` for the FTP server
- When `run` is called, the exploit runs and the payload runs a pretask which starts a listening TCP server for the shell connection
- The exploit is successful and the payload executes on the Metasploitable2 system, the listening TCP server receives a connection and a root shell is opened

[![asciicast](https://asciinema.org/a/SSTvtRi8cecmZvb687MjNkB1R.png)](https://asciinema.org/a/SSTvtRi8cecmZvb687MjNkB1R)

## TODO

- Module Types, catsploit currently has exploit and payload module types defined. Auxiliary, Evasion, Encoder, and many other module types should be created
- More modules, more exploits, payloads and others written so that catsploit becomes usable in a general pentest
- Better documentation for new users, tutorials, videos, etc.

## Automated Testing
Some points on automated testing within Catsploit:

- Tests are written for logical functionality. For example in `catsploit_lib/src/core/exploit/remote_tcp.rs`, tests are written for the both `connect` and `custom_connect` but not for `opts`. A test could be written for `opts` that iterates through the values looking for `RHOST` etc., but this makes the changing the `opts` function more involved for not much benefit
- Tests are not written for functions that only print to STDOUT with no side effects. Example being `print_exploit` in `catsploit/src/cli/info.rs`
- It's fine to modify a functions parameters and code block solely to make it more testable. For example `show_exploits` in `catsploit/src/cli/cmd/show.rs` takes a boolean indicating if the function is running in a test or not, to prevent it from printing the full exploit table to STDOUT during tests. There may be possible ways to block the STDOUT printing in the tests using closures etc., that wouldn't need to modify the `show_exploits` function signature. The added complexity and development time for that isn't worth it to avoid a simple parameter change.

### Running Tests
To run tests for both the catsploit library and the CLI application:

```
cargo test --manifest-path=catsploit_lib/Cargo.toml && cargo test --manifest-path=catsploit/Cargo.toml
```
