use crate::core::{exploit::Exploit, payload::Payload};

use crate::module::exploit::ftp::vsftpd_234_backdoor::Vsftpd234Backdoor;

use crate::module::payload::{
    linux_shell::nc_mkfifo_reverse_tcp::NcMkfifoReverseTcp, ruby::ruby_reverse_tcp::RubyReverseTcp,
};

pub fn exploits() -> Vec<Box<dyn Exploit>> {
    let mut exploits: Vec<Box<dyn Exploit>> = Vec::new();
    exploits.push(Box::new(Vsftpd234Backdoor::default()));
    exploits
}

pub fn payloads() -> Vec<Box<dyn Payload + Send + Sync>> {
    let mut payloads: Vec<Box<dyn Payload + Send + Sync>> = Vec::new();
    payloads.push(Box::new(RubyReverseTcp::default()));
    payloads.push(Box::new(NcMkfifoReverseTcp::default()));
    payloads
}
