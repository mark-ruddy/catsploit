use dyn_clone::DynClone;
use std::{error::Error, fmt};

use super::opt::Opt;

pub mod reverse;

#[derive(Debug)]
pub enum Kind {
    ReverseShell,
    DirectShell,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub struct Info {
    pub descriptive_name: String,
    pub module_path: String,
    pub kind: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub author: Option<Vec<String>>,
    pub references: Option<Vec<String>>,
    pub platform: Option<Vec<String>>,
}

// TODO: need a better way to define whether a payload has pretask or not
/// Run the pretask of any payload, will skip if the payload kind doesn't use pretask
/// Revshells for example may use a pretask to start the listener on the attacking machine
pub fn run_pretask(payload: Box<dyn Payload + Send + Sync>) -> Result<(), Box<dyn Error>> {
    match payload.kind() {
        Kind::ReverseShell => {
            // TODO: how to propogate error
            payload.pretask().unwrap();
        }
        _ => (),
    }
    Ok(())
}

pub trait Payload: DynClone {
    fn default() -> Self
    where
        Self: Sized;

    fn kind(&self) -> Kind {
        Kind::ReverseShell
    }

    fn info(&self) -> Info;

    fn pretask(&self) -> Result<(), Box<dyn Error>>;

    fn blob(&self) -> Vec<u8>;

    fn blob_to_string(&self) -> Result<String, Box<dyn Error>> {
        let blob = self.blob();
        match String::from_utf8(blob) {
            Ok(blob_string) => Ok(blob_string),
            Err(e) => Err(format!("failed to convert blob to UTF-8 string: {}", e).into()),
        }
    }

    fn blob_insert(&self, blob: Vec<u8>) -> Vec<u8> {
        // TODO: Blob insert for now does nothing except return the raw blob
        blob
    }

    fn opts(&self) -> Vec<Opt>;

    fn apply_opts(&mut self, opts: Vec<Opt>) -> Result<(), Box<dyn Error>>;
}

dyn_clone::clone_trait_object!(Payload);
