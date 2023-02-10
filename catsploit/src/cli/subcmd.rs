use catsploit_lib::module::Kind;

use crate::{info, show, use_cmd, util::parse_cliopts, MODULE_KINDS};
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
                "payloads" => show::payloads(),
                _ => return Err(SHOW_SUBCMD_INCORRECT.into()),
            },
            None => return Err(SHOW_SUBCMD_INCORRECT.into()),
        }
        Ok(())
    }

    pub fn handle_info(&self) -> Result<(), Box<dyn Error>> {
        // Display exploit info if module kind
        match &self.selected_module_kind {
            Some(selected_module_kind) => match selected_module_kind {
                // TODO: Investigate why these parameters must be references
                // TODO: Need to have opts stored in the CLI...can't just print what the library knows
                Kind::Exploit => {
                    info::print_exploit(&self.exploit_info, &self.selected_module_opts)?
                }
                Kind::Payload => {
                    info::print_payload(&self.payload_info, &self.selected_module_opts)?
                }
                Kind::Auxiliary => return Err("Auxiliary info not supported yet".into()),
            },
            None => return Err("Info requires a module to be selected".into()),
        }
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
                        self.prompt = Some(exploit_info.module_path.clone());
                        self.selected_module_kind = Some(Kind::Exploit);
                        self.selected_module_path = Some(exploit_info.module_path.clone());
                        self.exploit_info = Some(exploit_info);
                        self.selected_module_opts = Some(parse_cliopts(exploit.opts()));
                    }
                    "payload" => {
                        let payload = use_cmd::payload(&subcmd)?;
                        let payload_info = payload.info();
                        self.prompt = Some(payload_info.module_path.clone());
                        self.selected_module_kind = Some(Kind::Payload);
                        self.selected_module_path = Some(payload_info.module_path.clone());
                        self.payload_info = Some(payload_info);
                        self.selected_module_opts = Some(parse_cliopts(payload.opts()));
                    }
                    _ => (),
                }
            }
            None => return Err(NO_SUBCMD.into()),
        };
        Ok(())
    }
}
