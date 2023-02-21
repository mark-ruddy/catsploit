use catsploit_lib::{
    core::payload::Payload, module::payload::linux_shell::nc_mkfifo_reverse_tcp::NcMkfifoReverseTcp,
};

pub fn payload() -> Box<dyn Payload + Send + Sync> {
    // TODO: for now just declaring the default payload to be RubyReverseTcp
    return Box::new(NcMkfifoReverseTcp::default());
}
