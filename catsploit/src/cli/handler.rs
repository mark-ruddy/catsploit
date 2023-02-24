use super::{
    cmd::use_module::{find_exploit, find_payload},
    Cli,
};
use crate::MODULE_KINDS;
use catsploit_lib::module::Kind;
use std::error::Error;

const NO_SUBCMD: &str = "This command requires an argument";

struct ParsedModulePath {
    kind: Kind,
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
        let kind_enum = match *kind {
            "exploit" => Kind::Exploit,
            "payload" => Kind::Payload,
            "auxiliary" => Kind::Auxiliary,
            _ => return Err("Unknown kind in module path".into()),
        };
        Ok(ParsedModulePath { kind: kind_enum })
    }

    pub fn handle_show(&mut self, subcmd: Option<String>) -> Result<(), Box<dyn Error>> {
        const SHOW_SUBCMD_INCORRECT: &str = "Possible options for show are 'exploits', 'payloads'";
        match subcmd {
            Some(subcmd) => match subcmd.as_str() {
                "exploits" => self.show_exploits(false),
                "payloads" => self.show_payloads(false),
                _ => return Err(SHOW_SUBCMD_INCORRECT.into()),
            },
            None => return Err(SHOW_SUBCMD_INCORRECT.into()),
        }
        Ok(())
    }

    pub fn handle_info(&self, subcmd: Option<String>) -> Result<(), Box<dyn Error>> {
        match subcmd {
            Some(module_path) => {
                let parsed_module_path = self.parse_module_path(&module_path)?;
                match parsed_module_path.kind {
                    Kind::Exploit => {
                        let exploit = find_exploit(&module_path)
                            .ok_or("No exploit exists at module...TODO duplicate error messages")?;
                        self.print_exploit(&exploit.info())?;
                    }
                    Kind::Payload => {
                        let payload =
                            find_payload(&module_path).ok_or("No payload exists at module...")?;
                        self.print_payload(&payload.info())?;
                    }
                    Kind::Auxiliary => return Err("Auxiliary info not supported yet".into()),
                }
                return Ok(());
            }
            None => (),
        }

        match &self.selected_module_kind {
            Some(selected_module_kind) => match selected_module_kind {
                Kind::Exploit => self.print_exploit(
                    self.exploit_info
                        .as_ref()
                        .ok_or("No selected exploit module to display info on")?,
                )?,
                Kind::Payload => self.print_payload(
                    self.payload_info
                        .as_ref()
                        .ok_or("No selected payload module to display info on")?,
                )?,
                Kind::Auxiliary => return Err("Auxiliary info not supported yet".into()),
            },
            None => return Err("Info requires a module to be selected".into()),
        }
        Ok(())
    }

    pub fn handle_use(&mut self, subcmd: Option<String>) -> Result<(), Box<dyn Error>> {
        match subcmd {
            Some(subcmd) => {
                match subcmd.parse::<usize>() {
                    Ok(subcmd) => {
                        self.use_from_displayed_list(&subcmd)?;
                        return Ok(());
                    }
                    Err(_) => (),
                }
                self.handle_use_with_module_path(&subcmd)?;
            }
            None => return Err(NO_SUBCMD.into()),
        };
        Ok(())
    }

    pub fn handle_use_with_module_path(&mut self, module_path: &str) -> Result<(), Box<dyn Error>> {
        let parsed_module_path = self.parse_module_path(module_path)?;
        match parsed_module_path.kind {
            Kind::Exploit => self.use_exploit(module_path)?,
            Kind::Payload => self.use_payload(module_path)?,
            Kind::Auxiliary => return Err("Auxiliary info not supported yet".into()),
        }
        Ok(())
    }

    pub fn handle_set(
        &mut self,
        subcmd: Option<String>,
        args: Option<Vec<String>>,
    ) -> Result<(), Box<dyn Error>> {
        const MISSING_VALUE: &str = "Missing value argument";
        let subcmd = subcmd.ok_or("Missing option name argument")?;
        let args = args.ok_or(MISSING_VALUE)?;
        if args.len() < 1 {
            return Err(MISSING_VALUE.into());
        }

        // special case for "set payload"
        if subcmd == "payload" {
            self.set_payload(&args[0])?;
            return Ok(());
        }

        self.set_opt(&subcmd, &args[0])?;
        Ok(())
    }

    pub fn handle_run(&mut self) -> Result<(), Box<dyn Error>> {
        match &self.selected_module_kind {
            Some(selected_module_kind) => match selected_module_kind {
                Kind::Exploit => self.run_exploit()?,
                _ => return Err("Run is supported for 'exploit' modules only".into()),
            },
            None => return Err("Module kind is not set".into()),
        };
        Ok(())
    }

    pub fn handle_help(&self) {
        // TODO: need to make help look better with tables or smth
        println!(
            "# Core Commands \
            \n- show (Show available modules) \
            \n- info (Display information for current module) \
            \n- use  (Select a module) \
            \n- exit (Exit catsploit console)"
        );
    }
}
