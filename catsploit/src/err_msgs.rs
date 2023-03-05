// NOTE: not all error messages used in catsploit CLI are defined here, only the ones that occur multiple times in code
pub const NO_AUXILIARY_MODULE_INFO: &str = "No selected auxiliary module to display info on";
pub const NO_EXPLOIT_MODULE_INFO: &str = "No selected exploit module to display info on";
pub const NO_PAYLOAD_MODULE_INFO: &str = "No selected payload module to display info on";

pub fn no_auxiliaries_exist(module_path: &str) -> String {
    format!("No auxiliaries exists at module path '{}'", module_path)
}

pub fn no_exploits_exist(module_path: &str) -> String {
    format!("No exploits exists at module path '{}'", module_path)
}

pub fn no_payloads_exist(module_path: &str) -> String {
    format!("No payloads exists at module path '{}'", module_path)
}
