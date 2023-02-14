use crate::cli::Cli;
use std::error::Error;

impl Cli {
    pub fn set(&mut self, opt_name: &str, value: &str) -> Result<(), Box<dyn Error>> {
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
        Ok(())
    }
}
