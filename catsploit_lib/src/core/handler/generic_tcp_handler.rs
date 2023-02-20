const READ_POLLS: u64 = 3;
const READ_POLL_SLEEP_MILLIS: u64 = 50;

use log::info;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    time::Duration,
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
        let stream_buf = BufReader::new(stream.try_clone()?);
        Self::open_shell(stream, stream_buf)?;
        Ok(())
    }

    pub fn open_shell(
        mut stream: TcpStream,
        mut stream_buf: BufReader<TcpStream>,
    ) -> Result<(), Box<dyn Error>> {
        stream.set_nonblocking(true)?;
        loop {
            print!("> ");
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input == "catsploit_exit" {
                return Ok(());
            }
            stream.write_all(input.as_bytes())?;

            // TODO: try again by using the bufreader to iterate on lines, shell input comes out in lines so that is only reasonable way
            let mut read_poll_counter = 0;
            loop {
                let mut line = String::new();
                match stream_buf.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => print!("{}", line),
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // reading is blocked as no bytes coming from BufReader<TcpStream> right now
                        // wait a bit and try again until READ_POLLS reached
                        std::thread::sleep(Duration::from_millis(READ_POLL_SLEEP_MILLIS));
                        read_poll_counter += 1;
                        // println!("waiting read counter: {}", read_poll_counter);
                        if read_poll_counter == READ_POLLS {
                            break;
                        }
                    }
                    Err(e) => return Err(e.into()),
                }
            }
        }
    }
}
