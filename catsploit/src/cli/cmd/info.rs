use std::error::Error;

use crate::cli::Cli;

const NO_INFO: &str = "No module info loaded";

// TODO: need a consistent way to print tabled data without borders

impl Cli {
    pub fn print_exploit(&self) -> Result<(), Box<dyn Error>> {
        let info = match &self.exploit_info {
            Some(info) => info,
            None => return Err(NO_INFO.into()),
        };

        println!("EXPLOIT NAME: {}", info.descriptive_name);
        println!("EXPLOIT RANKING: {}", info.ranking);

        self.print_cliopts();

        Ok(())
    }

    pub fn print_payload(&self) -> Result<(), Box<dyn Error>> {
        let info = match &self.payload_info {
            Some(info) => info,
            None => return Err(NO_INFO.into()),
        };

        println!("PAYLOAD NAME: {}", info.descriptive_name);
        println!("PAYLOAD KIND: {}", info.kind);

        self.print_cliopts();

        Ok(())
    }
}
