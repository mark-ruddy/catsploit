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

                        // TODO: update previous module opts here
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
