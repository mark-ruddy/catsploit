use crate::core::opt::Opt;

#[derive(Clone)]
pub struct Reverse {
    pub lhost: String,
    pub lport: String,
}

impl Default for Reverse {
    fn default() -> Reverse {
        Reverse {
            lhost: "0.0.0.0".to_string(),
            // TODO: need to define a constant default sane port to use for listeners? need to have multiple?
            lport: "9092".to_string(),
        }
    }
}

impl Reverse {
    pub fn opts() -> Vec<Opt> {
        let mut opts: Vec<Opt> = Vec::new();
        opts.push(Opt {
            name: "LHOST".to_string(),
            description: "Listener host".to_string(),
            default_value: Some("0.0.0.0".to_string()),
            value: Some("0.0.0.0".to_string()),
        });
        opts.push(Opt {
            name: "LPORT".to_string(),
            description: "Listener port".to_string(),
            default_value: Some("9092".to_string()),
            value: Some("9092".to_string()),
        });
        opts
    }
}
