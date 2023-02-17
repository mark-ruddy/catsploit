use std::error::Error;

use crate::cli::Cli;

impl Cli {
    pub fn run_exploit(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(exploit) = &mut self.exploit {
            exploit.apply_opts(
                self.selected_module_opts
                    .clone()
                    .ok_or("No module options set to apply to exploit")?,
            )?;

            let mut exploit_payload = self
                .exploit_payload
                .clone()
                .ok_or("A payload must be defined to apply opts to")?;
            let exploit_payload_module_path = exploit_payload.info().module_path;

            match self.previous_module_opts.get(&exploit_payload_module_path) {
                Some(previous_module_opts) => {
                    exploit_payload.apply_opts(previous_module_opts.clone())?;
                }
                None => {
                    exploit_payload.apply_opts(exploit_payload.opts())?;
                }
            };
            println!(
                "IN LIBRARY AFTER APPLYING OPTS TO CLONED PAYLOAD: {}",
                exploit_payload.blob_to_string()?
            );

            exploit.exploit(&exploit_payload)?;
        }
        Ok(())
    }
}
