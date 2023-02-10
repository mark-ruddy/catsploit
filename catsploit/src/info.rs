use catsploit_lib::core::{exploit, opt::Opt, payload};
use std::error::Error;

use crate::opts::print_opts;

const NO_INFO: &str = "No module info loaded";

// TODO: need a consistent way to print tabled data without borders

pub fn print_exploit(
    info: &Option<exploit::Info>,
    opts: &Option<Vec<Opt>>,
) -> Result<(), Box<dyn Error>> {
    let info = match info {
        Some(info) => info,
        None => return Err(NO_INFO.into()),
    };

    println!("EXPLOIT NAME: {}", info.descriptive_name);
    println!("EXPLOIT RANKING: {}", info.ranking);

    print_opts(opts);

    Ok(())
}

pub fn print_payload(
    info: &Option<payload::Info>,
    opts: &Option<Vec<Opt>>,
) -> Result<(), Box<dyn Error>> {
    let info = match info {
        Some(info) => info,
        None => return Err(NO_INFO.into()),
    };

    println!("PAYLOAD NAME: {}", info.descriptive_name);
    println!("PAYLOAD KIND: {}", info.kind);

    print_opts(opts);

    Ok(())
}
