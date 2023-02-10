use crate::cli::CliOpt;
use catsploit_lib::core::opt::Opt;

pub fn parse_cliopts(opts: Vec<Opt>) -> Vec<CliOpt> {
    let mut cliopts: Vec<CliOpt> = Vec::new();
    for opt in opts {
        let value = opt.default_value.clone();
        cliopts.push(CliOpt { opt, value })
    }
    cliopts
}
