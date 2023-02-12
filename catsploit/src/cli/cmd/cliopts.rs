use catsploit_lib::core::opt::Opt;
use prettytable::Table;

use crate::cli::{Cli, CliOpt};

impl Cli {
    pub fn parse_cliopts(opts: Vec<Opt>) -> Vec<CliOpt> {
        let mut cliopts: Vec<CliOpt> = Vec::new();
        for opt in opts {
            let value = opt.default_value.clone();
            cliopts.push(CliOpt { opt, value })
        }
        cliopts
    }

    pub fn print_cliopts(&self) {
        match &self.selected_module_cliopts {
            Some(selected_module_cliopts) => {
                let mut opts_table = Table::new();
                opts_table.add_row(row!["Name", "Description", "Default", "Current"]);
                for cliopt in selected_module_cliopts {
                    let default_value = cliopt.opt.default_value.clone().unwrap_or("".to_string());
                    let value = cliopt.value.clone().unwrap_or("".to_string());
                    opts_table.add_row(row![
                        cliopt.opt.name,
                        cliopt.opt.description,
                        default_value,
                        value,
                    ]);
                }
                opts_table.printstd();
            }
            None => (),
        }
    }
}
