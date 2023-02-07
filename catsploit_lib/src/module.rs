pub mod index;

pub mod auxiliary;
pub mod exploit;
pub mod payload;

pub enum Kind {
    Auxiliary,
    Exploit,
    Payload,
}
