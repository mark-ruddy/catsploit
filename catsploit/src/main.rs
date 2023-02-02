use log::{error, info};
use std::time::Duration;

use catsploit_lib::core::exploit::remote_tcp::RemoteTcp;
use catsploit_lib::core::exploit::Exploit;
use catsploit_lib::module::exploit::ftp::vsftpd_234_backdoor::Vsftpd234Backdoor;

mod cli;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    cli::show_exploits();

    /*
    println!("lets exploit vsftpd v2.3.4");
    println!(
        "Vsftpd exploit description: {}",
        Vsftpd234Backdoor::info().description
    );
    let vsftpd_234_backdoor = Vsftpd234Backdoor::new(
        RemoteTcp {
            rhost: "127.0.0.1".to_string(),
            rport: "21".to_string(),
            read_timeout: Some(Duration::from_secs(60)),
            write_timeout: Some(Duration::from_secs(60)),
        },
        None,
    );
    match vsftpd_234_backdoor.exploit() {
        Ok(_) => (),
        Err(e) => {
            error!("{}", e);
        }
    }
    */

    // TODO: basic CLI interface, lets just code, can refactor later
}
