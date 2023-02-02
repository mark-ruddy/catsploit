// use std::{io, io::Write};
use std::process;

use crate::show;

pub fn print_banner() {
    println!("---- CATSPLOIT ----");
}

pub fn print_prompt(content: Option<&str>) {
    match content {
        Some(content) => print!("catsploit ({})>", content),
        None => print!("catsploit> "),
    }
}

pub fn get_input() -> 

pub fn handle_input(input: &str) {
    match input {
        "show exploits" => show::exploits(),
        "help" => println!("No help supported yet"),
        "exit" => {
            println!("Exiting...");
            process::exit(0);
        }
        _ => println!("Unknown command '{}'", input),
    }
}
