use prettytable::Table;
use std::error::Error;

use crate::cli::Cli;

impl Cli {
    pub fn print_opts(&self) {
        match &self.selected_module_opts {
            Some(selected_module_opts) => {
                let mut opts_table = Table::new();
                opts_table.add_row(row!["Name", "Description", "Default", "Current"]);
                for opt in selected_module_opts {
                    let default_value = opt.default_value.clone().unwrap_or("".to_string());
                    let value = opt.value.clone().unwrap_or("".to_string());
                    opts_table.add_row(row![opt.name, opt.description, default_value, value,]);
                }
                opts_table.printstd();
            }
            None => (),
        }
    }

    #[allow(dead_code)]
    pub fn get_opt_value(&self, name: &str) -> Result<Option<String>, Box<dyn Error>> {
        match &self.selected_module_opts {
            Some(selected_module_opts) => {
                for opt in selected_module_opts {
                    if opt.name == name {
                        match opt.value.clone() {
                            Some(value) => return Ok(Some(value)),
                            None => return Ok(None),
                        }
                    }
                }
                return Err(format!(
                    "No option with name '{}' found in selected module options",
                    name
                )
                .into());
            }
            None => return Err("No selected module options to get value from".into()),
        }
    }

    pub fn update_previous_module_opts(&mut self) -> Result<(), Box<dyn Error>> {
        self.previous_module_opts.insert(
            self.selected_module_path
                .clone()
                .ok_or("No selected module path to get opts for")?,
            self.selected_module_opts
                .clone()
                .ok_or("No selected module options to update with")?,
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPLOIT_MODULE_PATH: &str = "exploit/ftp/vsftpd_234_backdoor";
    const PAYLOAD_MODULE_PATH: &str = "payload/linux_shell/nc_mkfifo_reverse_tcp";

    #[test]
    fn test_get_opt_value() {
        let mut cli = Cli::default();
        cli.use_exploit(EXPLOIT_MODULE_PATH).unwrap();
        cli.set_opt("RHOST", "8.8.8.8").unwrap();
        assert_eq!(cli.get_opt_value("RHOST").unwrap().unwrap(), "8.8.8.8");
    }

    #[test]
    fn test_get_opt_value_non_existant() {
        let mut cli = Cli::default();
        cli.use_exploit(EXPLOIT_MODULE_PATH).unwrap();
        match cli.get_opt_value("DOES_NOT_EXIST") {
            Ok(_) => panic!("Expected option value to not exist"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_update_previous_module_opts() {
        let mut cli = Cli::default();
        cli.use_exploit(EXPLOIT_MODULE_PATH).unwrap();
        cli.set_opt("RHOST", "8.8.8.8").unwrap();

        cli.use_payload(PAYLOAD_MODULE_PATH).unwrap();
        cli.set_opt("LHOST", "1.1.1.1").unwrap();

        cli.use_exploit(EXPLOIT_MODULE_PATH).unwrap();
        assert_eq!(cli.get_opt_value("RHOST").unwrap().unwrap(), "8.8.8.8");

        cli.use_payload(PAYLOAD_MODULE_PATH).unwrap();
        assert_eq!(cli.get_opt_value("LHOST").unwrap().unwrap(), "1.1.1.1");
    }
}
