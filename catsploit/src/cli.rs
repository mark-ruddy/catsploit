use std::{error::Error, io, process};

use catsploit_lib::{
    core::{exploit, opt::Opt, payload},
    module::Kind,
};

mod cmd;
mod handler;

/// CliOpt holds a value assigned by the user to a library option
pub struct CliOpt {
    pub opt: Opt,
    pub value: Option<String>,
}

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

pub struct Cli {
    pub prompt: Option<String>,
    pub selected_module_kind: Option<Kind>,
    pub selected_module_path: Option<String>,
    pub selected_module_cliopts: Option<Vec<CliOpt>>,

    pub exploit_info: Option<exploit::Info>,
    pub payload_info: Option<payload::Info>,
}

impl Cli {
    pub fn print_banner(&self) {
        println!("---- CATSPLOIT ----");
    }

    pub fn print_prompt(&self) {
        match &self.prompt {
            Some(prompt) => print!("catsploit ({})> ", prompt),
            None => print!("catsploit> "),
        }
    }

    pub fn get_user_input(&self) -> Result<UserInput, Box<dyn Error>> {
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

    pub fn handle_input(&mut self, input: UserInput) -> Result<(), Box<dyn Error>> {
        match input.cmd.as_str() {
            "show" => self.handle_show(input.subcmd)?,
            "info" => self.handle_info()?,
            "use" => self.handle_use(input.subcmd)?,
            "help" => self.handle_help(),
            "exit" => {
                println!("Exiting...");
                process::exit(0);
            }
            _ => {
                if input.cmd != "" {
                    println!("Unknown command '{}'", input.cmd);
                }
            }
        };
        Ok(())
    }
}
