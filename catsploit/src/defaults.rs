use catsploit_lib::{core::payload::Payload, module::payload::ruby_reverse_tcp::RubyReverseTcp};

pub fn payload() -> Box<dyn Payload + Send + Sync> {
    // TODO: for now just declaring the default payload to be RubyReverseTcp
    return Box::new(RubyReverseTcp::default());
}
