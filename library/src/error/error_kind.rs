
/// Kinds of NetCommsError that can arise in this library.
#[derive(Debug)]
pub enum NetCommsErrorKind {
    WrongCommand,   
    InvalidPacketKind, 
    SerializingFailed,
    DeserializingFailed,
    InvalidBufferLength,
    /// Wrapper around every error not originating from this library.
    OtherSource(Box<dyn std::error::Error>),
}

impl std::fmt::Display for NetCommsErrorKind {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NetCommsErrorKind")
    }    
}