use crate::{core::exploit::Exploit, module::exploit::ftp::vsftpd_234_backdoor::Vsftpd234Backdoor};

pub fn exploits() -> Vec<Box<dyn Exploit>> {
    let mut exploits: Vec<Box<dyn Exploit>> = Vec::new();
    exploits.push(Box::new(Vsftpd234Backdoor::default()));
    exploits
}
