use crate::{module_index, show, MODULE_KINDS};
use std::error::Error;

const NO_SUBCMD: &str = "This command requires an argument";

struct ParsedModulePath {
    kind: String,
}

pub fn handle_show(subcmd: Option<String>) -> Result<(), Box<dyn Error>> {
    const SHOW_SUBCMD_INCORRECT: &str = "Possible options for show are 'exploits', 'payloads'";
    match subcmd {
        Some(subcmd) => match subcmd.as_str() {
            "exploits" => show::exploits(),
            _ => return Err(SHOW_SUBCMD_INCORRECT.into()),
        },
        None => return Err(SHOW_SUBCMD_INCORRECT.into()),
    }
    Ok(())
}

fn parse_module_path(module_path: &str) -> Result<ParsedModulePath, Box<dyn Error>> {
    let split = module_path.split("/");
    let split_vec: Vec<&str> = split.collect();

    let mut kind = match split_vec.get(0) {
        Some(kind) => kind,
        None => return Err("Invalid module path, no kind present".into()),
    };
    if !MODULE_KINDS.contains(kind) {
        return Err(format!("Unknown module kind '{}' in module path", kind).into());
    }
    Ok(ParsedModulePath {
        kind: kind.to_string(),
    })
}

fn use_exploit(module_path: &str) -> Result<Option<String>, Box<dyn Error>> {
    let exploits = module_index::exploit::exploits();
    let mut found = false;
    for exploit in exploits {
        if exploit.info().module_path == module_path {
            found = true;
        }
    }
    if !found {
        return Err(format!("No exploit found with the module path '{}'", module_path).into());
    }
    Ok(Some(module_path.to_string()))
}

// TODO: For now just assuming that the subcmd is the module path, need to implement selecting by number but that requires "search" to be implemented and history
pub fn handle_use(subcmd: Option<String>) -> Result<Option<String>, Box<dyn Error>> {
    const USE_SUBCMD_INCORRECT: &str = "'{}' does not exist to use";
    let prompt_change = match subcmd {
        Some(subcmd) => {
            let parsed_module_path = parse_module_path(&subcmd)?;
            match parsed_module_path.kind.as_str() {
                "exploit" => use_exploit(&subcmd)?,
                _ => None,
            }
        }
        None => return Err(NO_SUBCMD.into()),
    };
    Ok(prompt_change)
}
