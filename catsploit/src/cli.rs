use std::{error::Error, io, process};

use self::subcmd::{handle_show, handle_use};

mod subcmd;

pub struct UserInput {
    pub cmd: String,
    pub subcmd: Option<String>,
    pub args: Option<String>,
}

impl Default for UserInput {
    fn default() -> Self {
        UserInput {
            cmd: "default".to_string(),
            subcmd: None,
            args: None,
        }
    }
}

pub fn print_banner() {
    println!("---- CATSPLOIT ----");
}

pub fn print_prompt(content: &Option<String>) {
    match content {
        Some(content) => print!("catsploit ({})> ", content),
        None => print!("catsploit> "),
    }
}

pub fn get_user_input() -> Result<UserInput, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();
    let split = input.split(" ");
    let split_vec: Vec<&str> = split.collect();

    let mut user_input = UserInput::default();
    match split_vec.get(0) {
        Some(cmd) => user_input.cmd = cmd.trim().to_string(),
        None => return Err("No command passed in user input".into()),
    }

    match split_vec.get(1) {
        Some(subcmd) => user_input.subcmd = Some(subcmd.trim().to_string()),
        None => {
            user_input.subcmd = None;
            return Ok(user_input);
        }
    }

    match split_vec.get(2) {
        Some(_) => (),
        None => {
            user_input.args = None;
            return Ok(user_input);
        }
    }
    let args_vec = &split_vec[2..];

    user_input.args = Some(args_vec.join(" "));
    Ok(user_input)
}

pub fn handle_input(input: UserInput) -> Result<Option<String>, Box<dyn Error>> {
    let prompt_change = match input.cmd.as_str() {
        "show" => {
            handle_show(input.subcmd)?;
            None
        }
        "use" => handle_use(input.subcmd)?,
        "help" => {
            println!("No help supported yet");
            None
        }
        "exit" => {
            println!("Exiting...");
            process::exit(0);
        }
        _ => {
            println!("Unknown command '{}'", input.cmd);
            None
        }
    };
    Ok(prompt_change)
}
