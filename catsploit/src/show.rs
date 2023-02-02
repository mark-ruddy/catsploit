use crate::module_index;
use catsploit_lib::core::exploit;

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

fn print_exploit_show_info(info: ExploitShowInfo) {
    println!("{:?}", info);
}

pub fn exploits() {
    let exploits = module_index::exploit::exploits();
    for exploit in exploits {
        let info = extract_exploit_show_info(exploit.info());
        print_exploit_show_info(info);
    }
}
