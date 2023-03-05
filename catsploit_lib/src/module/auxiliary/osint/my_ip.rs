use crate::core::auxiliary::{Auxiliary, Info, Kind};
use reqwest::blocking::get;
use std::error::Error;

#[derive(Clone)]
pub struct MyIp {}

impl MyIp {
    pub fn new() -> MyIp {
        MyIp {}
    }
}

impl Auxiliary for MyIp {
    fn default() -> Self {
        MyIp {}
    }

    fn kind(&self) -> Kind {
        Kind::Osint
    }

    fn info(&self) -> Info {
        Info {
            descriptive_name: "My IP".to_string(),
            module_path: "auxiliary/osint/my_ip".to_string(),
            kind: self.kind().to_string(),
            description: "Print your public IPv4 or IPv6 address by querying the SeeIP API"
                .to_string(),
            license: None,
            author: None,
            references: None,
            platform: None,
        }
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let response = get("https://ip.seeip.org/")?.text()?;
        println!("{}", response);
        Ok(())
    }
}
