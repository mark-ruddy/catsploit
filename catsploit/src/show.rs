use crate::module_index;
use catsploit_lib::core::exploit;
use prettytable::Table;

#[derive(Debug)]
struct ExploitShowInfo {
    name: String,
    module_path: String,
    ranking: String,
}

fn extract_exploit_show_info(info: exploit::Info) -> ExploitShowInfo {
    ExploitShowInfo {
        name: info.descriptive_name,
        module_path: info.module_path,
        ranking: info.ranking,
    }
}

pub fn exploits() {
    let exploits = module_index::exploit::exploits();

    let mut table = Table::new();
    table.add_row(row!["#", "Module Path", "Name", "Ranking"]);
    for (i, exploit) in exploits.iter().enumerate() {
        let info = extract_exploit_show_info(exploit.info());
        table.add_row(row![i, info.module_path, info.name, info.ranking]);
    }
    table.printstd();
}
