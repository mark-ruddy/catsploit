use crate::cli::Cli;
use catsploit_lib::core::payload::Payload;
use catsploit_lib::module::index;
use std::error::Error;

impl Cli {
    pub fn set_opt(&mut self, opt_name: &str, value: &str) -> Result<(), Box<dyn Error>> {
        let mut found = false;
        match &mut self.selected_module_opts {
            Some(selected_module_opts) => {
                for opt in selected_module_opts.iter_mut() {
                    if opt.name == opt_name {
                        opt.value = Some(value.to_string());
                        found = true;
                    }
                }
            }
            None => return Err("No current module options to set".into()),
        }
        if !found {
            return Err(format!("No module option with name '{}' found", opt_name).into());
        }
        self.update_previous_module_opts()?;
        Ok(())
    }

    pub fn set_payload(&mut self, module_path: &str) -> Result<(), Box<dyn Error>> {
        // TODO: code adapted from use_module.rs, maybe some can be extracted to fn
        let payloads = index::payloads();
        let mut selected_payload: Option<Box<dyn Payload + Send + Sync>> = None;
        for payload in payloads {
            if payload.info().module_path == module_path {
                selected_payload = Some(payload);
            }
        }
        match selected_payload {
            Some(payload) => {
                self.exploit_payload = Some(payload);
                Ok(())
            }
            None => Err(format!("No payload found with the module path '{}'", module_path).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use catsploit_lib::core::payload;

    const EXPLOIT_MODULE_PATH: &str = "exploit/ftp/vsftpd_234_backdoor";
    const PAYLOAD_MODULE_PATH: &str = "payload/linux_shell/nc_mkfifo_reverse_tcp";
    const PAYLOAD_MODULE_PATH_NON_EXISTANT: &str = "payload/linux_shell/does_not_exist";

    #[test]
    fn test_set_opt() {
        let mut cli = Cli::default();
        // will use an exploit that uses RemoteTcp, in this case the option RHOST should be present
        cli.use_exploit(EXPLOIT_MODULE_PATH).unwrap();
        cli.set_opt("RHOST", "8.8.8.8").unwrap();
        let selected_module_opts = cli.selected_module_opts.unwrap();
        let mut found = false;
        for opt in selected_module_opts {
            if opt.name == "RHOST" {
                found = true;
                assert_eq!(opt.value.unwrap(), "8.8.8.8");
            }
        }
        if !found {
            panic!("Expected RHOST option to be present in selected module options");
        }
    }

    #[test]
    fn test_set_opt_non_existant() {
        let mut cli = Cli::default();
        cli.use_exploit(EXPLOIT_MODULE_PATH).unwrap();
        cli.set_opt("DOES_NOT_EXIST", "value")
            .expect_err("set_opt should not be successful for non existant option");
    }

    #[test]
    fn test_set_payload() {
        let mut cli = Cli::default();
        cli.use_exploit(EXPLOIT_MODULE_PATH).unwrap();
        cli.set_payload(PAYLOAD_MODULE_PATH).unwrap();
        let payload = cli.exploit_payload.unwrap();
        assert_eq!(payload.kind(), payload::Kind::ReverseShell);
        assert_eq!(payload.info().descriptive_name, "Netcat Mkfifo Reverse TCP");
    }

    #[test]
    fn test_set_payload_non_existant() {
        let mut cli = Cli::default();
        cli.use_exploit(EXPLOIT_MODULE_PATH).unwrap();
        match cli.set_payload(PAYLOAD_MODULE_PATH_NON_EXISTANT) {
            Ok(_) => (),
            Err(e) => assert!(e.to_string().contains("No payload found")),
        }
    }
}
