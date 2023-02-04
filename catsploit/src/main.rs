#[macro_use]
extern crate prettytable;
use cli::Cli;
use std::{error::Error, io, io::Write};

mod cli;
mod show;
mod use_cmd;

const MODULE_KINDS: [&str; 2] = ["exploit", "payload"];

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // TODO: basic CLI interface, lets just code, can refactor later
    let mut cli = Cli { prompt: None };
    cli.print_banner();
    loop {
        cli.print_prompt();
        io::stdout().flush()?;

        let user_input = match cli.get_user_input() {
            Ok(user_input) => user_input,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        match cli.handle_input(user_input) {
            Ok(prompt_change) => prompt_change,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
    }
}
