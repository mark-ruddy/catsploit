use catsploit_lib::core::{exploit::Exploit, payload::Payload};
use catsploit_lib::module::{index, Kind};
use std::error::Error;

use crate::cli::Cli;

impl Cli {
    pub fn use_exploit(&mut self, module_path: &str) -> Result<Box<dyn Exploit>, Box<dyn Error>> {
        let exploits = index::exploits();
        let mut selected_exploit: Option<Box<dyn Exploit>> = None;
        for exploit in exploits {
            if exploit.info().module_path == module_path {
                selected_exploit = Some(exploit);
            }
        }
        match selected_exploit {
            Some(exploit) => {
                let exploit_info = exploit.info();
                self.prompt = Some(exploit_info.module_path.clone());
                self.selected_module_kind = Some(Kind::Exploit);
                self.selected_module_path = Some(exploit_info.module_path.clone());
                self.exploit_info = Some(exploit_info);
                self.selected_module_cliopts = Some(Cli::parse_cliopts(exploit.opts()));
                return Ok(exploit);
            }
            None => {
                return Err(
                    format!("No exploit found with the module path '{}'", module_path).into(),
                )
            }
        }
    }

    pub fn use_payload(&mut self, module_path: &str) -> Result<Box<dyn Payload>, Box<dyn Error>> {
        let payloads = index::payloads();
        let mut selected_payload: Option<Box<dyn Payload>> = None;
        for payload in payloads {
            if payload.info().module_path == module_path {
                selected_payload = Some(payload);
            }
        }
        match selected_payload {
            Some(payload) => {
                let payload_info = payload.info();
                self.prompt = Some(payload_info.module_path.clone());
                self.selected_module_kind = Some(Kind::Payload);
                self.selected_module_path = Some(payload_info.module_path.clone());
                self.payload_info = Some(payload_info);
                self.selected_module_cliopts = Some(Cli::parse_cliopts(payload.opts()));
            }
            None => {
                return Err(
                    format!("No payload found with the module path '{}'", module_path).into(),
                )
            }
        }
        return Err(format!("No payload found with the module path '{}'", module_path).into());
    }
}
