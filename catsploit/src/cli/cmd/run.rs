use std::error::Error;

use catsploit_lib::module::Kind;

use crate::cli::Cli;

impl Cli {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        match &self.selected_module_kind {
            Some(selected_module_kind) => match selected_module_kind {
                Kind::Exploit => match &mut self.exploit {
                    Some(exploit) => {
                        // Apply the options set in the CLI and then run the exploit
                        exploit.apply_opts(
                            self.selected_module_opts
                                .clone()
                                .ok_or("No module options set to apply to exploit")?,
                        )?;
                        /*
                        let exploit_payload = match self.exploit_payload {
                            Some(exploit_payload) => exploit_payload,
                            None => return Err("A payload must be defined to run exploit")?,
                        };
                        */
                        exploit.exploit(
                            // TODO: why is as_ref needed here vs using &self?
                            /* dyn_clone::clone_box(exploit_payload),
                            self.exploit_payload
                                .ok_or("A payload must be defined to run exploit")?,
                            */
                            &self.exploit_payload,
                        )?;
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
