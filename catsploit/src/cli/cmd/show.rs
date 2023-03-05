use catsploit_lib::{
    core::{auxiliary, exploit, payload},
    module::index,
};
use prettytable::{format, Table};

use crate::cli::Cli;

#[derive(Debug)]
struct AuxiliaryShowInfo {
    name: String,
    module_path: String,
    kind: String,
}

#[derive(Debug)]
struct ExploitShowInfo {
    name: String,
    module_path: String,
    ranking: String,
}

#[derive(Debug)]
struct PayloadShowInfo {
    name: String,
    module_path: String,
    kind: String,
}

fn extract_auxiliary_show_info(info: auxiliary::Info) -> AuxiliaryShowInfo {
    AuxiliaryShowInfo {
        name: info.descriptive_name,
        module_path: info.module_path,
        kind: info.kind,
    }
}

fn extract_exploit_show_info(info: exploit::Info) -> ExploitShowInfo {
    ExploitShowInfo {
        name: info.descriptive_name,
        module_path: info.module_path,
        ranking: info.ranking,
    }
}

fn extract_payload_show_info(info: payload::Info) -> PayloadShowInfo {
    PayloadShowInfo {
        name: info.descriptive_name,
        module_path: info.module_path,
        kind: info.kind,
    }
}

impl Cli {
    pub fn print_module_stats(&self) {
        let auxiliaries = index::auxiliary();
        let exploits = index::exploits();
        let payloads = index::payloads();
        let mut table = Table::new();
        table.add_row(row!["Module Type", "Loaded"]);
        table.add_row(row!["Exploits", exploits.len()]);
        table.add_row(row!["Payloads", payloads.len()]);
        table.add_row(row!["Auxiliaries", auxiliaries.len()]);
        table.set_format(*format::consts::FORMAT_NO_COLSEP);
        table.printstd();
    }

    pub fn show_auxiliary(&mut self, test: bool) {
        let auxiliaries = index::auxiliary();
        let mut table = Table::new();
        table.add_row(row!["#", "Module Path", "Name", "Ranking"]);
        self.displayed_list.clear();
        for (i, auxiliary) in auxiliaries.iter().enumerate() {
            let info = extract_auxiliary_show_info(auxiliary.info());
            table.add_row(row![i, info.module_path, info.name, info.kind]);
            self.displayed_list.insert(i, info.module_path);
        }
        if !test {
            table.printstd();
        }
    }

    pub fn show_exploits(&mut self, test: bool) {
        let exploits = index::exploits();
        let mut table = Table::new();
        table.add_row(row!["#", "Module Path", "Name", "Ranking"]);
        self.displayed_list.clear();
        for (i, exploit) in exploits.iter().enumerate() {
            let info = extract_exploit_show_info(exploit.info());
            table.add_row(row![i, info.module_path, info.name, info.ranking]);
            self.displayed_list.insert(i, info.module_path);
        }
        if !test {
            table.printstd();
        }
    }

    pub fn show_payloads(&mut self, test: bool) {
        let payloads = index::payloads();
        let mut table = Table::new();
        table.add_row(row!["#", "Module Path", "Name", "Kind"]);
        self.displayed_list.clear();
        for (i, payload) in payloads.iter().enumerate() {
            let info = extract_payload_show_info(payload.info());
            table.add_row(row![i, info.module_path, info.name, info.kind]);
            self.displayed_list.insert(i, info.module_path);
        }
        if !test {
            table.printstd();
        }
    }
}
