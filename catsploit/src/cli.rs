use super::defaults;
use catsploit_lib::{
    core::{
        exploit::{self, Exploit},
        opt::Opt,
        payload::{self, Payload},
    },
    module::{index, Kind},
};
use prettytable::{format, Table};
use std::{collections::HashMap, error::Error, io, process};

mod cmd;
mod handler;

pub struct UserInput {
    pub cmd: String,
    pub subcmd: Option<String>,
    pub args: Option<Vec<String>>,
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
    pub selected_module_opts: Option<Vec<Opt>>,
    pub previous_module_opts: HashMap<String, Vec<Opt>>,
    pub displayed_list: HashMap<usize, String>,

    pub exploit: Option<Box<dyn Exploit>>,
    pub exploit_info: Option<exploit::Info>,
    pub exploit_payload: Option<Box<dyn Payload + Send + Sync>>,

    pub payload: Option<Box<dyn Payload + Send + Sync>>,
    pub payload_info: Option<payload::Info>,
}

impl Default for Cli {
    fn default() -> Cli {
        Cli {
            prompt: None,
            selected_module_kind: None,
            selected_module_path: None,
            selected_module_opts: None,
            previous_module_opts: HashMap::new(),
            displayed_list: HashMap::new(),

            exploit: None,
            exploit_info: None,
            exploit_payload: Some(defaults::payload()),

            payload: None,
            payload_info: None,
        }
    }
}

impl Cli {
    pub fn print_banner(&self) {
        println!(
            r#"
 ________  ________  _________  ________  ________  ___       ________  ___  _________   
|\   ____\|\   __  \|\___   ___\\   ____\|\   __  \|\  \     |\   __  \|\  \|\___   ___\ 
\ \  \___|\ \  \|\  \|___ \  \_\ \  \___|\ \  \|\  \ \  \    \ \  \|\  \ \  \|___ \  \_| 
 \ \  \    \ \   __  \   \ \  \ \ \_____  \ \   ____\ \  \    \ \  \\\  \ \  \   \ \  \  
  \ \  \____\ \  \ \  \   \ \  \ \|____|\  \ \  \___|\ \  \____\ \  \\\  \ \  \   \ \  \ 
   \ \_______\ \__\ \__\   \ \__\  ____\_\  \ \__\    \ \_______\ \_______\ \__\   \ \__\
    \|_______|\|__|\|__|    \|__| |\_________\|__|     \|_______|\|_______|\|__|    \|__|
                                  \|_________|                                           
            "#
        )
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
        let split_vec: Vec<String> = split.map(|s| s.to_string()).collect();

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
        let args_vec = args_vec.to_vec();

        if args_vec.len() > 0 {
            user_input.args = Some(args_vec);
        } else {
            user_input.args = None;
        }
        Ok(user_input)
    }

    pub fn handle_input(&mut self, input: UserInput) -> Result<(), Box<dyn Error>> {
        match input.cmd.as_str() {
            "show" => self.handle_show(input.subcmd)?,
            "info" => self.handle_info(input.subcmd)?,
            "use" => self.handle_use(input.subcmd)?,
            "set" => self.handle_set(input.subcmd, input.args)?,
            "run" => self.handle_run()?,
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
