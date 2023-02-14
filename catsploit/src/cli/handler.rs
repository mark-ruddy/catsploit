use super::Cli;
use crate::MODULE_KINDS;
use catsploit_lib::module::Kind;
use std::error::Error;

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
                "exploits" => Cli::show_exploits(),
                "payloads" => Cli::show_payloads(),
                _ => return Err(SHOW_SUBCMD_INCORRECT.into()),
            },
            None => return Err(SHOW_SUBCMD_INCORRECT.into()),
        }
        Ok(())
    }

    pub fn handle_info(&self) -> Result<(), Box<dyn Error>> {
        match &self.selected_module_kind {
            Some(selected_module_kind) => match selected_module_kind {
                Kind::Exploit => self.print_exploit()?,
                Kind::Payload => self.print_payload()?,
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
                    "exploit" => self.use_exploit(&subcmd)?,
                    "payload" => self.use_payload(&subcmd)?,
                    "options" => self.print_opts(),
                    "opts" => self.print_opts(),
                    _ => (),
                }
            }
            None => return Err(NO_SUBCMD.into()),
        };
        Ok(())
    }

    pub fn handle_set(
        &mut self,
        subcmd: Option<String>,
        args: Option<Vec<String>>,
    ) -> Result<(), Box<dyn Error>> {
        const MISSING_VALUE: &str = "Missing value argument";
        let opt_name = subcmd.ok_or("Missing option name argument")?;
        let args = args.ok_or(MISSING_VALUE)?;
        if args.len() < 1 {
            return Err(MISSING_VALUE.into());
        }
        self.set(&opt_name, &args[0])?;
        Ok(())
    }

    pub fn handle_run(&mut self) -> Result<(), Box<dyn Error>> {
        self.run()?;
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
