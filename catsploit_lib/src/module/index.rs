use crate::core::auxiliary::Auxiliary;
use crate::core::{exploit::Exploit, payload::Payload};

use crate::module::auxiliary::osint::my_ip::MyIp;

use crate::module::exploit::ftp::vsftpd_234_backdoor::Vsftpd234Backdoor;

use crate::module::payload::{
    linux_shell::nc_mkfifo_reverse_tcp::NcMkfifoReverseTcp, ruby::ruby_reverse_tcp::RubyReverseTcp,
};

pub fn exploits() -> Vec<Box<dyn Exploit>> {
    vec![Box::new(Vsftpd234Backdoor::default())]
}

pub fn payloads() -> Vec<Box<dyn Payload + Send + Sync>> {
    vec![
        Box::new(RubyReverseTcp::default()),
        Box::new(NcMkfifoReverseTcp::default()),
    ]
}

pub fn auxiliary() -> Vec<Box<dyn Auxiliary + Send + Sync>> {
    vec![Box::new(MyIp::default())]
}
