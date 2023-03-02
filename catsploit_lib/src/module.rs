pub mod auxiliary;
pub mod exploit;
pub mod payload;

pub mod index;

#[derive(PartialEq, Eq, Debug)]
pub enum Kind {
    Auxiliary,
    Exploit,
    Payload,
}
