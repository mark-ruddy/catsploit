use std::error::Error;

use catsploit_lib::module::Kind;

use crate::cli::Cli;

impl Cli {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match &self.selected_module_kind {
            Some(selected_module_kind) => match selected_module_kind {
                Kind::Exploit => match &self.exploit {
                    Some(exploit) => {
                        // TODO: Need a way to map the Vec<CliOpt> values into the concrete underlying object(e.g. Vsftpd234Backdoor) that implements Exploit - not easy
                        exploit.exploit()?;
                    }
                    None => return Err("Exploit module is not set correctly".into()),
                },
                // NOTE: payloads do not support run
                _ => return Err("Run is supported for 'exploit' modules only".into()),
            },
            None => return Err("Module kind is not set".into()),
        }
        Ok(())
    }
}
