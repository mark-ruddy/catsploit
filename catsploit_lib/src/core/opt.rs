// TODO: What the cli needs from the library: The option name, its description, default value and mandatory status
// by having a default_value declared, its implied that its not mandatory for the user to change it

pub struct Opt {
    pub name: String,
    pub description: String,
    pub default_value: Option<String>,
}
