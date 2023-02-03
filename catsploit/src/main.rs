#[macro_use]
extern crate prettytable;
use cli::{get_user_input, handle_input, print_banner, print_prompt};
use std::{error::Error, io, io::Write};

mod cli;
mod module_index;
mod show;

const MODULE_KINDS: [&str; 2] = ["exploit", "payload"];

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

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
    let mut prompt_change = None;
    print_banner();
    loop {
        print_prompt(&prompt_change);
        io::stdout().flush()?;

        let user_input = match get_user_input() {
            Ok(user_input) => user_input,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        prompt_change = match handle_input(user_input) {
            Ok(prompt_change) => prompt_change,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
    }
}
