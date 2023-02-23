use catsploit_lib::core::{exploit::Exploit, payload::Payload};
use catsploit_lib::module::{index, Kind};
use std::error::Error;

use crate::cli::Cli;

pub fn find_exploit(module_path: &str) -> Option<Box<dyn Exploit>> {
    let exploits = index::exploits();
    let mut selected_exploit: Option<Box<dyn Exploit>> = None;
    for exploit in exploits {
        if exploit.info().module_path == module_path {
            selected_exploit = Some(exploit);
        }
    }
    selected_exploit
}

pub fn find_payload(module_path: &str) -> Option<Box<dyn Payload + Send + Sync>> {
    let payloads = index::payloads();
    let mut selected_payload: Option<Box<dyn Payload + Send + Sync>> = None;
    for payload in payloads {
        if payload.info().module_path == module_path {
            selected_payload = Some(payload);
        }
    }
    selected_payload
}

impl Cli {
    pub fn use_exploit(&mut self, module_path: &str) -> Result<(), Box<dyn Error>> {
        let selected_exploit = find_exploit(module_path);
        match selected_exploit {
            Some(exploit) => {
                let exploit_info = exploit.info();
                self.prompt = Some(exploit_info.module_path.clone());
                self.selected_module_kind = Some(Kind::Exploit);
                self.selected_module_path = Some(exploit_info.module_path.clone());

                // TODO: below code block is near duplicated in use_payload, might be acceptable in this case
                match self.previous_module_opts.get(module_path) {
                    Some(previous_module_opts) => {
                        self.selected_module_opts = Some(previous_module_opts.clone())
                    }
                    None => self.selected_module_opts = Some(exploit.opts()),
                }

                self.exploit = Some(exploit);
                self.exploit_info = Some(exploit_info);
                Ok(())
            }
            None => Err(format!("No exploit found with the module path '{}'", module_path).into()),
        }
    }

    pub fn use_payload(&mut self, module_path: &str) -> Result<(), Box<dyn Error>> {
        let selected_payload = find_payload(module_path);
        match selected_payload {
            Some(payload) => {
                let payload_info = payload.info();
                self.prompt = Some(payload_info.module_path.clone());
                self.selected_module_kind = Some(Kind::Payload);
                self.selected_module_path = Some(payload_info.module_path.clone());

                match self.previous_module_opts.get(module_path) {
                    Some(previous_module_opts) => {
                        self.selected_module_opts = Some(previous_module_opts.clone())
                    }
                    None => self.selected_module_opts = Some(payload.opts()),
                }

                self.payload = Some(payload);
                self.payload_info = Some(payload_info);
                Ok(())
            }
            None => Err(format!("No payload found with the module path '{}'", module_path).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use catsploit_lib::core::{exploit::Ranking, payload};

    const EXPLOIT_MODULE_PATH: &str = "exploit/ftp/vsftpd_234_backdoor";
    const EXPLOIT_MODULE_PATH_NON_EXISTING: &str = "exploit/ftp/does_not_exist";

    const PAYLOAD_MODULE_PATH: &str = "payload/linux_shell/nc_mkfifo_reverse_tcp";
    const PAYLOAD_MODULE_PATH_NON_EXISTING: &str = "payload/linux_shell/does_not_exist";

    #[test]
    fn test_find_exploit() {
        let exploit = find_exploit(EXPLOIT_MODULE_PATH).unwrap();
        assert_eq!(exploit.ranking(), Ranking::Excellent);
    }

    #[test]
    fn test_find_exploit_non_existing() {
        match find_exploit(EXPLOIT_MODULE_PATH_NON_EXISTING) {
            Some(_) => panic!("Exploit was found for a non existant module path"),
            None => (),
        }
    }

    #[test]
    fn test_find_payload() {
        let payload = find_payload(PAYLOAD_MODULE_PATH).unwrap();
        assert_eq!(payload.kind(), payload::Kind::ReverseShell);
    }

    #[test]
    fn test_find_payload_non_existing() {
        match find_payload(PAYLOAD_MODULE_PATH_NON_EXISTING) {
            Some(_) => panic!("Payload was found for a non existant module path"),
            None => (),
        }
    }

    #[test]
    fn test_use_exploit() {
        let mut cli = Cli::default();
        cli.use_exploit(EXPLOIT_MODULE_PATH).unwrap();
        assert_eq!(cli.selected_module_kind.unwrap(), Kind::Exploit);
        assert_eq!(cli.selected_module_path.unwrap(), EXPLOIT_MODULE_PATH);
        assert_eq!(
            cli.exploit_info.unwrap().descriptive_name,
            "VSFTPD v2.3.4 Backdoor Command Execution"
        );
        assert_eq!(cli.exploit.unwrap().ranking(), Ranking::Excellent);
    }

    #[test]
    fn test_use_exploit_non_existing() {
        let mut cli = Cli::default();
        match cli.use_exploit(EXPLOIT_MODULE_PATH_NON_EXISTING) {
            Ok(_) => (),
            Err(e) => assert!(e.to_string().contains("No exploit found")),
        }
    }

    #[test]
    fn test_use_payload() {
        let mut cli = Cli::default();
        cli.use_payload(PAYLOAD_MODULE_PATH).unwrap();
        assert_eq!(cli.selected_module_kind.unwrap(), Kind::Payload);
        assert_eq!(cli.selected_module_path.unwrap(), PAYLOAD_MODULE_PATH);
        assert_eq!(
            cli.payload_info.unwrap().descriptive_name,
            "Netcat Mkfifo Reverse TCP"
        );
        assert_eq!(cli.payload.unwrap().kind(), payload::Kind::ReverseShell);
    }

    #[test]
    fn test_use_payload_non_existing() {
        let mut cli = Cli::default();
        match cli.use_payload(PAYLOAD_MODULE_PATH_NON_EXISTING) {
            Ok(_) => (),
            Err(e) => assert!(e.to_string().contains("No payload found")),
        }
    }
}
