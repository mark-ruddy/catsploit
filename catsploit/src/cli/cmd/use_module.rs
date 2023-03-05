use catsploit_lib::core::{auxiliary::Auxiliary, exploit::Exploit, payload::Payload};
use catsploit_lib::module::{index, Kind};
use std::error::Error;

use crate::cli::Cli;

pub fn find_auxiliary(module_path: &str) -> Option<Box<dyn Auxiliary + Send + Sync>> {
    let auxiliaries = index::auxiliary();
    let mut selected_auxiliary: Option<Box<dyn Auxiliary + Send + Sync>> = None;
    for auxiliary in auxiliaries {
        if auxiliary.info().module_path == module_path {
            selected_auxiliary = Some(auxiliary);
        }
    }
    selected_auxiliary
}

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
    pub fn use_from_displayed_list(&mut self, index: &usize) -> Result<(), Box<dyn Error>> {
        match self.displayed_list.get(index) {
            Some(module_path) => {
                self.handle_use_with_module_path(module_path.clone().as_str())?;
                Ok(())
            }
            None => Err(format!(
                "Last displayed module list does not have an entry for '{}'",
                index
            )
            .into()),
        }
    }

    pub fn use_auxiliary(&mut self, module_path: &str) -> Result<(), Box<dyn Error>> {
        let selected_auxiliary = find_auxiliary(module_path);
        match selected_auxiliary {
            Some(auxiliary) => {
                let auxiliary_info = auxiliary.info();
                self.prompt = Some(auxiliary_info.module_path.clone());
                self.selected_module_kind = Some(Kind::Auxiliary);
                self.selected_module_path = Some(auxiliary_info.module_path.clone());

                match self.apply_previous_module_opts(module_path) {
                    true => (),
                    false => self.selected_module_opts = auxiliary.opts(),
                }

                self.auxiliary = Some(auxiliary);
                self.auxiliary_info = Some(auxiliary_info);
                Ok(())
            }
            None => {
                Err(format!("No auxiliary found with the module path '{}'", module_path).into())
            }
        }
    }

    pub fn use_exploit(&mut self, module_path: &str) -> Result<(), Box<dyn Error>> {
        let selected_exploit = find_exploit(module_path);
        match selected_exploit {
            Some(exploit) => {
                let exploit_info = exploit.info();
                self.prompt = Some(exploit_info.module_path.clone());
                self.selected_module_kind = Some(Kind::Exploit);
                self.selected_module_path = Some(exploit_info.module_path.clone());

                match self.apply_previous_module_opts(module_path) {
                    true => (),
                    false => self.selected_module_opts = exploit.opts(),
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

                match self.apply_previous_module_opts(module_path) {
                    true => (),
                    false => self.selected_module_opts = payload.opts(),
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
    const EXPLOIT_MODULE_PATH_NON_EXISTANT: &str = "exploit/ftp/does_not_exist";

    const PAYLOAD_MODULE_PATH: &str = "payload/linux_shell/nc_mkfifo_reverse_tcp";
    const PAYLOAD_MODULE_PATH_NON_EXISTANT: &str = "payload/linux_shell/does_not_exist";

    #[test]
    fn test_find_exploit() {
        let exploit = find_exploit(EXPLOIT_MODULE_PATH).unwrap();
        assert_eq!(exploit.ranking(), Ranking::Excellent);
    }

    #[test]
    fn test_find_exploit_non_existant() {
        assert!(
            find_exploit(EXPLOIT_MODULE_PATH_NON_EXISTANT).is_none(),
            "Exploit was found for a non existant module path"
        );
    }

    #[test]
    fn test_find_payload() {
        let payload = find_payload(PAYLOAD_MODULE_PATH).unwrap();
        assert_eq!(payload.kind(), payload::Kind::ReverseShell);
    }

    #[test]
    fn test_find_payload_non_existant() {
        assert!(
            find_payload(PAYLOAD_MODULE_PATH_NON_EXISTANT).is_none(),
            "Payload was found for a non existant module path"
        )
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
    fn test_use_exploit_non_existant() {
        let mut cli = Cli::default();
        match cli.use_exploit(EXPLOIT_MODULE_PATH_NON_EXISTANT) {
            Ok(_) => panic!("Use exploit should not be successful for non existant module path"),
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
    fn test_use_payload_non_existant() {
        let mut cli = Cli::default();
        match cli.use_payload(PAYLOAD_MODULE_PATH_NON_EXISTANT) {
            Ok(_) => panic!("Use payload should not be successful for non existant module path"),
            Err(e) => assert!(e.to_string().contains("No payload found")),
        }
    }

    #[test]
    fn test_use_from_displayed_list() {
        let mut cli = Cli::default();
        // the first exploit shown in show_exploits is expected to be vsftpd_234_backdoor with index 0
        cli.show_exploits(true);
        // handle_use should parse that the subcmd is a number and call use_from_displayed_list
        cli.handle_use(Some("0".to_string())).unwrap();
        assert_eq!(cli.selected_module_kind.unwrap(), Kind::Exploit);
        assert_eq!(cli.selected_module_path.unwrap(), EXPLOIT_MODULE_PATH);
        assert_eq!(
            cli.exploit_info.unwrap().descriptive_name,
            "VSFTPD v2.3.4 Backdoor Command Execution"
        );
        assert_eq!(cli.exploit.unwrap().ranking(), Ranking::Excellent);
    }

    #[test]
    fn test_use_from_displayed_list_non_existant() {
        let mut cli = Cli::default();
        cli.show_exploits(true);
        match cli.handle_use(Some("999999".to_string())) {
            Ok(_) => panic!("Handle use should not be successful for non existant module index"),
            Err(e) => {
                println!("ERR: {}", e);
                assert!(e
                    .to_string()
                    .contains("Last displayed module list does not have an entry for"));
            }
        }
    }
}
