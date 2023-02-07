use catsploit_lib::module::Kind;

use crate::{show, use_cmd, MODULE_KINDS};
use std::error::Error;

use super::Cli;

const NO_SUBCMD: &str = "This command requires an argument";

struct ParsedModulePath {
    kind: String,
}

// TODO: For now just assuming that the subcmd is the module path, need to implement selecting by number but that requires "search" to be implemented and history
impl Cli {
    fn parse_module_path(&self, module_path: &str) -> Result<ParsedModulePath, Box<dyn Error>> {
        let split = module_path.split("/");
        let split_vec: Vec<&str> = split.collect();

        let kind = match split_vec.get(0) {
            Some(kind) => kind,
            None => return Err("Invalid module path, no kind present".into()),
        };
        if !MODULE_KINDS.contains(kind) {
            return Err(format!("Unknown module kind '{}' in module path", kind).into());
        }
        Ok(ParsedModulePath {
            kind: kind.to_string(),
        })
    }

    pub fn handle_show(&self, subcmd: Option<String>) -> Result<(), Box<dyn Error>> {
        const SHOW_SUBCMD_INCORRECT: &str = "Possible options for show are 'exploits', 'payloads'";
        match subcmd {
            Some(subcmd) => match subcmd.as_str() {
                "exploits" => show::exploits(),
                _ => return Err(SHOW_SUBCMD_INCORRECT.into()),
            },
            None => return Err(SHOW_SUBCMD_INCORRECT.into()),
        }
        Ok(())
    }

    pub fn handle_info(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn handle_use(&mut self, subcmd: Option<String>) -> Result<(), Box<dyn Error>> {
        match subcmd {
            Some(subcmd) => {
                let parsed_module_path = self.parse_module_path(&subcmd)?;
                match parsed_module_path.kind.as_str() {
                    "exploit" => {
                        let exploit = use_cmd::exploit(&subcmd)?;
                        let exploit_info = exploit.info();
                        self.prompt = Some(exploit_info.module_path);
                        self.selected_module_kind = Some(Kind::Exploit);
                        self.selected_module_path = Some(exploit_info.module_path);
                        self.exploit_info = Some(exploit_info);
                    }
                    _ => (),
                }
            }
            None => return Err(NO_SUBCMD.into()),
        };
        Ok(())
    }
}
