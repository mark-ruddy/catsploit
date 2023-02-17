// TODO: I think what I want here is a generic TCP listener that can be launched just before a payload is sent over
// The server itself probably needs to run in its own thread, so it doesn't block the rest of the library executing
// Single client should be accepted, don't see any reason to want multiple clients for a revshell
// Need to be able to attach the servers I/O to the terminal too, that logic maybe can be implemented in handler.rs

use log::info;
use std::error::Error;
use std::io::{self, BufRead, BufReader};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub struct GenericTcpHandler {
    pub listener: TcpListener,
}

impl GenericTcpHandler {
    pub fn new(address: &str, port: &str) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind(format!("{}:{}", address, port))?;
        Ok(Self { listener })
    }

    pub fn listen_for_one(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: need timeout here while accepting
        info!(
            "Listening for one connection on: {}",
            self.listener.local_addr()?
        );
        let (stream, peer_addr) = self.listener.accept()?;
        info!("Received handler connection from: {}", peer_addr);
        Self::open_shell(stream)?;
        Ok(())
    }

    pub fn open_shell(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0; 1024];
        loop {
            // TODO: work in progress, trying to emulate nc -lvnp
            /*
            let mut cmd = String::new();
            io::stdin().read_line(&mut cmd)?;
            if cmd == "catsploit_handler_exit" {
                break;
            }
            println!("Got cmd, trying to write to stream: {}", cmd);
            stream.write(cmd.as_bytes())?;
            println!("written to stream");

            let mut out = String::new();
            stream_buf.read(&mut out)?;
            print!("{}", out);
            io::stdout().flush()?;
            */
            print!("> ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            stream.write_all(input.trim().as_bytes()).unwrap();

            match stream.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    let received = String::from_utf8_lossy(&buffer[0..n]);
                    println!("Received: {}", received.trim());
                }
                Err(e) => {
                    eprintln!("Error reading from socket: {}", e);
                    break;
                }
            }
        }
        Ok(())
    }
}
