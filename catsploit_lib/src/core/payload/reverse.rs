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
            lport: "9092".to_string(),
        }
    }
}

impl Reverse {
    pub fn opts() -> Vec<Opt> {
        vec![
            Opt {
                name: "LHOST".to_string(),
                description: "Listener host".to_string(),
                default_value: Some("0.0.0.0".to_string()),
                value: Some("0.0.0.0".to_string()),
            },
            Opt {
                name: "LPORT".to_string(),
                description: "Listener port".to_string(),
                default_value: Some("9092".to_string()),
                value: Some("9092".to_string()),
            },
        ]
    }
}

macro_rules! apply_opts {
    ($self:ident, $opts:expr) => {
        for opt in $opts {
            match opt.name.as_str() {
                "LHOST" => {
                    let lhost = opt.value.ok_or("LHOST option is required")?;
                    $self.reverse.lhost = lhost;
                }
                "LPORT" => $self.reverse.lport = opt.value.ok_or("LPORT option is required")?,
                _ => (),
            }
        }
    };
}
pub(crate) use apply_opts;
