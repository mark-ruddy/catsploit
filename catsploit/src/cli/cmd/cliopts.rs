use catsploit_lib::core::opt::Opt;

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
            Some(cliopts) => {
                for cliopt in cliopts {
                    match &cliopt.opt.default_value {
                        Some(default_value) => match &cliopt.value {
                            Some(value) => println!(
                                "{} - {}. DEFAULT {}. CURRENT {}.",
                                cliopt.opt.name, cliopt.opt.description, default_value, value
                            ),
                            None => println!(
                                "{} - {}. DEFAULT {}.",
                                cliopt.opt.name, cliopt.opt.description, default_value
                            ),
                        },
                        None => println!("{} - {}", cliopt.opt.name, cliopt.opt.description),
                    }
                }
            }
            None => return,
        }
    }
}
