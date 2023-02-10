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

// TODO: need a way for exploits to pick a default payload, similarly exploits need to be searchable etc too.
pub trait Payload {
    fn default() -> Self
    where
        Self: Sized;

    fn kind(&self) -> Kind {
        Kind::ReverseShell
    }

    fn info(&self) -> Info;

    /// Payloads may need to carry out a task before executing
    /// Revshells for example may use a pretask to start the listener on the attacking machine
    fn pretask(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

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
}
