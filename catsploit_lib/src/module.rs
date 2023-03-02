pub mod index;

pub mod auxiliary;
pub mod exploit;
pub mod payload;

#[derive(PartialEq, Eq, Debug)]
pub enum Kind {
    Auxiliary,
    Exploit,
    Payload,
}
