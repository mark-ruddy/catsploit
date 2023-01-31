use std::error::Error;

pub mod reverse;

pub enum Kind {
    Single,
    Adapter,
}

// TODO: need a way for exploits to pick a default payload, similarly exploits need to be searchable etc too.
pub trait Payload {
    fn kind() -> Kind {
        Kind::Single
    }

    /// Payloads may need to carry out a task before executing
    /// Revshells for example may use a pretask to start the listener on the attacking machine
    fn pretask(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn blob(&self) -> Vec<u8>;

    fn blob_as_string(&self) -> Result<String, Box<dyn Error>> {
        let blob = self.blob();
        match String::from_utf8(blob) {
            Ok(blob_string) => Ok(blob_string),
            Err(e) => Err(format!("failed to convert blob to UTF8 string: {}", e).into()),
        }
    }

    fn blob_insert(blob: Vec<u8>) -> Vec<u8> {
        // TODO: Blob insert for now does nothing except return the raw blob
        blob
    }
}
