use cli::{handle_input, print_banner, print_prompt};
use std::{error::Error, io, io::Write};

mod cli;
mod module_index;
mod show;

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
    print_banner();
    loop {
        print_prompt(None);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        handle_input(&input.trim());
    }
}
