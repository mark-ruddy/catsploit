use catsploit_lib::core::opt::Opt;
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
