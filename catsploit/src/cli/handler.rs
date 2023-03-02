use super::{
    cmd::use_module::{find_exploit, find_payload},
    Cli,
};
use crate::{err_msgs, MODULE_KINDS};
use catsploit_lib::module::Kind;
use prettytable::Table;
use std::error::Error;

const NO_SUBCMD: &str = "This command requires an argument";

struct ParsedModulePath {
    kind: Kind,
}

pub struct CommandHelp {
    pub name: String,
    pub description: String,
    pub usage: String,
}

impl Cli {
    fn parse_module_path(&self, module_path: &str) -> Result<ParsedModulePath, Box<dyn Error>> {
        let split = module_path.split('/');
        let split_vec: Vec<&str> = split.collect();

        let kind = match split_vec.first() {
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
        if let Some(module_path) = subcmd {
            let parsed_module_path = self.parse_module_path(&module_path)?;
            match parsed_module_path.kind {
                Kind::Exploit => {
                    let exploit = find_exploit(&module_path)
                        .ok_or_else(|| err_msgs::no_exploits_exist(&module_path))?;
                    self.print_exploit(&exploit.info())?;
                }
                Kind::Payload => {
                    let payload = find_payload(&module_path)
                        .ok_or_else(|| err_msgs::no_payloads_exist(&module_path))?;
                    self.print_payload(&payload.info())?;
                }
                Kind::Auxiliary => return Err("Auxiliary info not supported yet".into()),
            }
            return Ok(());
        }

        match &self.selected_module_kind {
            Some(selected_module_kind) => match selected_module_kind {
                Kind::Exploit => self.print_exploit(
                    self.exploit_info
                        .as_ref()
                        .ok_or(err_msgs::NO_EXPLOIT_MODULE_INFO)?,
                )?,
                Kind::Payload => self.print_payload(
                    self.payload_info
                        .as_ref()
                        .ok_or(err_msgs::NO_PAYLOAD_MODULE_INFO)?,
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
                if let Ok(subcmd) = subcmd.parse::<usize>() {
                    self.use_from_displayed_list(&subcmd)?;
                    return Ok(());
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
        if args.is_empty() {
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

    pub fn print_specific_help(command_help_vec: Vec<CommandHelp>) {
        let mut table = Table::new();
        table.add_row(row!["Command", "Description", "Usage"]);
        for command_help in command_help_vec {
            table.add_row(row![
                command_help.name,
                command_help.description,
                command_help.usage
            ]);
        }
        table.printstd();
    }

    pub fn handle_help() {
        let core_command_help: Vec<CommandHelp> = vec![
            CommandHelp {
                name: "modules".to_string(),
                description: "Display information on loaded modules".to_string(),
                usage: "modules".to_string(),
            },
            CommandHelp {
                name: "show".to_string(),
                description: "Show all modules of a kind".to_string(),
                usage: "show exploits | show payloads".to_string(),
            },
            CommandHelp {
                name: "use".to_string(),
                description: "Select a module".to_string(),
                usage: "use exploit/ftp/vsftpd_234_backdoor | show exploits > use 0".to_string(),
            },
            CommandHelp {
                name: "info".to_string(),
                description: "Display information on a selected module or specified module"
                    .to_string(),
                usage: "info | info exploit/ftp/vsftpd_234_backdoor".to_string(),
            },
            CommandHelp {
                name: "set".to_string(),
                description: "Set an option in a selected module or set a payload to use for a selected exploit".to_string(),
                usage: "use exploit/ftp/vsftpd_234_backdoor > set RHOST 8.8.8.8 | use exploit/ftp/vsftpd_234_backdoor > set payload payload/ruby/reverse_tcp".to_string(),
            },
            CommandHelp {
                name: "exit".to_string(),
                description: "Exit Catsploit".to_string(),
                usage: "exit".to_string(),
            }
        ];
        Cli::print_specific_help(core_command_help);
    }
}
