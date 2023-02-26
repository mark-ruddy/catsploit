#[derive(Clone)]
pub struct Opt {
    pub name: String,
    pub description: String,
    pub default_value: Option<String>,
    pub value: Option<String>,
}
