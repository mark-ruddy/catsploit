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
