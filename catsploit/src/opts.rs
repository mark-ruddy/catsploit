use crate::cli::CliOpt;

pub fn print_cliopts(cliopts: &Option<Vec<CliOpt>>) {
    match cliopts {
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
