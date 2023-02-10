use catsploit_lib::core::opt::Opt;

pub fn print_opts(opts: &Option<Vec<Opt>>) {
    match opts {
        Some(opts) => {
            for opt in opts {
                match &opt.default_value {
                    Some(default_value) => {
                        println!(
                            "{} - {}. DEFAULT: {}",
                            opt.name, opt.description, default_value
                        )
                    }
                    None => println!("{} - {}", opt.name, opt.description),
                }
            }
        }
        None => return,
    }
}
