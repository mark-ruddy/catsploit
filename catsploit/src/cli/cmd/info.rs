use catsploit_lib::core::{auxiliary, exploit, payload};
use prettytable::Table;
use std::error::Error;

use crate::cli::Cli;

impl Cli {
    pub fn print_auxiliary(&self, info: &auxiliary::Info) -> Result<(), Box<dyn Error>> {
        let mut auxiliary_table = Table::new();
        auxiliary_table.add_row(row!["Name", "Module Path", "Kind", "Description"]);
        auxiliary_table.add_row(row![
            info.descriptive_name,
            info.module_path,
            info.kind,
            info.description
        ]);
        auxiliary_table.printstd();

        self.print_opts();
        Ok(())
    }

    pub fn print_exploit(&self, info: &exploit::Info) -> Result<(), Box<dyn Error>> {
        let mut exploit_table = Table::new();
        exploit_table.add_row(row![
            "Name",
            "Module Path",
            "Disclosure Date",
            "Kind",
            "Ranking",
            "Description"
        ]);
        exploit_table.add_row(row![
            info.descriptive_name,
            info.module_path,
            info.disclosure_date,
            info.kind,
            info.ranking,
            info.description
        ]);
        exploit_table.printstd();

        self.print_opts();

        let exploit_payload_name = match &self.exploit_payload {
            Some(exploit_payload) => exploit_payload.info().descriptive_name,
            None => "".to_string(),
        };
        let mut exploit_payload_table = Table::new();
        exploit_payload_table.add_row(row!["Selected Payload", exploit_payload_name]);
        exploit_payload_table.printstd();

        Ok(())
    }

    pub fn print_payload(&self, info: &payload::Info) -> Result<(), Box<dyn Error>> {
        let mut payload_table = Table::new();
        if let Some(description) = info.description.clone() {
            payload_table.add_row(row!["Name", "Module Path", "Kind", "Description"]);
            payload_table.add_row(row![
                info.descriptive_name,
                info.module_path,
                info.kind,
                description
            ]);
        } else {
            payload_table.add_row(row!["Name", "Module Path", "Kind"]);
            payload_table.add_row(row![info.descriptive_name, info.module_path, info.kind,]);
        }
        payload_table.printstd();

        self.print_opts();
        Ok(())
    }
}
