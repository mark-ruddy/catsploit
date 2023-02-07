use catsploit_lib::core::exploit::Exploit;
use catsploit_lib::module::index;
use std::error::Error;

pub fn exploit(module_path: &str) -> Result<Box<dyn Exploit>, Box<dyn Error>> {
    let exploits = index::exploits();
    for exploit in exploits {
        if exploit.info().module_path == module_path {
            return Ok(exploit);
        }
    }
    return Err(format!("No exploit found with the module path '{}'", module_path).into());
}
