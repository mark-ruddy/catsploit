use dyn_clone::DynClone;
use std::{error::Error, fmt::Display};

use super::opt::Opt;

#[derive(Debug, Display)]
pub enum Kind {
    Scanner,
    Osint,
}

pub struct Info {
    pub descriptive_name: String,
    pub module_path: String,
    pub kind: String,
    pub description: String,
    pub license: Option<String>,
    pub author: Option<Vec<String>>,
    pub references: Option<Vec<String>>,
    pub platform: Option<Vec<String>>,
}

pub trait Auxiliary: DynClone {
    fn default() -> Self
    where
        Self: Sized;

    fn kind(&self) -> Kind;

    fn needs_pretask(&self) -> bool {
        false
    }

    fn pretask(&self) -> Result<(), Box<dyn Error>> {
        Err("Unimplemented pretask".into())
    }

    fn info(&self) -> Info;

    fn opts(&self) -> Option<Vec<Opt>> {
        None
    }

    #[allow(unused_variables)]
    fn apply_opts(&mut self, opts: Vec<Opt>) -> Result<(), Box<dyn Error>> {
        Err("Unimplemented apply_opts".into())
    }

    fn run(&self) -> Result<(), Box<dyn Error>>;
}

dyn_clone::clone_trait_object!(Auxiliary);
