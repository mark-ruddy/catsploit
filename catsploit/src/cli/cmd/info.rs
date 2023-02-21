use catsploit_lib::core::{exploit, payload};
use prettytable::Table;
use std::error::Error;

use crate::cli::Cli;

impl Cli {
    pub fn print_exploit(&self, info: &exploit::Info) -> Result<(), Box<dyn Error>> {
        // let info = info.ok_or(NO_INFO)?;
        let mut exploit_table = Table::new();
        exploit_table.add_row(row![
            "Name",
            "Module Path",
            "Disclosure Date",
            "Kind",
            "Ranking"
        ]);
        exploit_table.add_row(row![
            info.descriptive_name,
            info.module_path,
            info.disclosure_date,
            info.kind,
            info.ranking
        ]);
        exploit_table.printstd();

        self.print_opts();

        match &self.exploit_payload {
            Some(exploit_payload) => println!(
                "Selected Payload: {}",
                exploit_payload.info().descriptive_name
            ),
            None => (),
        }
        Ok(())
    }

    pub fn print_payload(&self, info: &payload::Info) -> Result<(), Box<dyn Error>> {
        let mut payload_table = Table::new();
        payload_table.add_row(row!["Name", "Module Path", "Kind"]);
        payload_table.add_row(row![info.descriptive_name, info.module_path, info.kind]);
        payload_table.printstd();

        self.print_opts();
        Ok(())
    }
}
