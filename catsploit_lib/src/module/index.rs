use crate::core::{exploit::Exploit, payload::Payload};

use crate::module::exploit::ftp::vsftpd_234_backdoor::Vsftpd234Backdoor;

use crate::module::payload::ruby_reverse_tcp::RubyReverseTcp;

pub fn exploits() -> Vec<Box<dyn Exploit>> {
    let mut exploits: Vec<Box<dyn Exploit>> = Vec::new();
    exploits.push(Box::new(Vsftpd234Backdoor::default()));
    exploits
}

pub fn payloads() -> Vec<Box<dyn Payload>> {
    let mut payloads: Vec<Box<dyn Payload>> = Vec::new();
    payloads.push(Box::new(RubyReverseTcp::default()));
    payloads
}
