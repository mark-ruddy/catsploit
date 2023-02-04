use catsploit_lib::module::index;
use std::error::Error;

pub fn exploit(module_path: &str) -> Result<(), Box<dyn Error>> {
    let exploits = index::exploits();
    let mut found = false;
    for exploit in exploits {
        if exploit.info().module_path == module_path {
            found = true;
        }
    }
    if !found {
        return Err(format!("No exploit found with the module path '{}'", module_path).into());
    }
    Ok(())
}
