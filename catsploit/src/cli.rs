use catsploit_lib::core::exploit;
use catsploit_lib::core::exploit::Exploit;
use catsploit_lib::module::exploit::ftp::vsftpd_234_backdoor::Vsftpd234Backdoor;

#[derive(Debug)]
struct ExploitShowInfo {
    name: String,
    module_path: String,
    ranking: String,
}

fn exploit_show_info(info: exploit::Info) -> ExploitShowInfo {
    ExploitShowInfo {
        name: info.descriptive_name,
        module_path: info.module_path,
        ranking: info.ranking,
    }
}

pub fn show_exploits() {
    println!("{:?}", exploit_show_info(Vsftpd234Backdoor::info()));
}
