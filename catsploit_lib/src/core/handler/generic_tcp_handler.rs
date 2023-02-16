// TODO: I think what I want here is a generic TCP listener that can be launched just before a payload is sent over
// The server itself probably needs to run in its own thread, so it doesn't block the rest of the library executing
// Single client should be accepted, don't see any reason to want multiple clients for a revshell
// Need to be able to attach the servers I/O to the terminal too, that logic maybe can be implemented in handler.rs

use log::info;
use std::error::Error;
use std::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub struct GenericTcpHandler {
    pub listener: TcpListener,
}

impl GenericTcpHandler {
    pub async fn new(address: &str, port: &str) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;
        Ok(Self { listener })
    }

    pub async fn listen_for_one(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: need timeout here while accepting
        info!(
            "Listening for one connection on: {}",
            self.listener.local_addr()?
        );
        let (stream, peer_addr) = self.listener.accept().await?;
        info!("Received handler connection from: {}", peer_addr);
        Self::open_shell(stream).await?;
        Ok(())
    }

    pub async fn open_shell(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        loop {
            let mut cmd = String::new();
            print!("shell_input> ");
            io::stdin().read_line(&mut cmd)?;
            if cmd == "catsploit_handler_exit" {
                break;
            }
            stream.write(cmd.as_bytes()).await?;

            let mut out = String::new();
            stream.read_to_string(&mut out).await?;
            print!("{}", out);
        }
        Ok(())
    }
}
