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

    pub fn listen_for_one(&mut self, test: bool) -> Result<(), Box<dyn Error>> {
        info!(
            "Listening for one connection on: {}",
            self.listener.local_addr()?
        );
        let (stream, peer_addr) = self.listener.accept()?;
        info!("Received handler connection from: {}", peer_addr);
        if test {
            return Ok(());
        }

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
            if input.trim() == "catsploit_exit" {
                return Ok(());
            }
            stream.write_all(input.as_bytes())?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    const HOST: &str = "127.0.0.1";
    const PORT: &str = "40347";

    fn setup_test_handler() -> GenericTcpHandler {
        GenericTcpHandler::new(HOST, PORT).unwrap()
    }

    #[test]
    fn test_accepts_one() {
        let server_thread = thread::spawn(|| {
            let mut handler = setup_test_handler();
            handler.listen_for_one(true).unwrap();
        });

        let client_thread = thread::spawn(|| {
            thread::sleep(Duration::from_millis(50));
            let addr = format!("{}:{}", HOST, PORT);
            TcpStream::connect(&addr).unwrap();
        });

        server_thread.join().unwrap();
        client_thread.join().unwrap();
    }
}
