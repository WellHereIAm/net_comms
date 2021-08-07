use crate::error::NetCommsErrorKind;


/// Error struct for this library.
// #[derive(Debug)]
pub struct NetCommsError {
    pub kind: NetCommsErrorKind,
    pub message: Option<String>,
}

impl std::fmt::Debug for NetCommsError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Self::output(&self, f)        
    }    
}

impl std::fmt::Display for NetCommsError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Self::output(&self, f)        
    }    
}

impl std::error::Error for NetCommsError {}

impl NetCommsError {

    fn output(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self.kind {
            NetCommsErrorKind::WrongCommand => {
                match &self.message {
                    Some(message) => write!(f, "NetCommsError(Wrong Command):\n {}", message),
                    None => write!(f, "Wrong Command"),
                }
            },
            NetCommsErrorKind::InvalidPacketKind => {
                match &self.message {
                    Some(message) => write!(f, "NetCommsError(Invalid Packet Kind):\n {}", message),
                    None => write!(f, "Invalid Packet Kind"),
                }
            },
            NetCommsErrorKind::SerializingFailed => {
                match &self.message {
                    Some(message) => write!(f, "NetCommsError(Serializing Failed):\n {}", message),
                    None => write!(f, "Serializing Failed"),
                }
            },
            NetCommsErrorKind::DeserializingFailed => {
                match &self.message {
                    Some(message) => write!(f, "NetCommsError(Deserializing Failed):\n {}", message),
                    None => write!(f, "Deserializing Failed"),
                }
            },
            NetCommsErrorKind::InvalidBufferLength => {
                match &self.message {
                    Some(message) => write!(f, "NetCommsError(Invalid Buffer Length):\n {}", message),
                    None => write!(f, "Invalid Buffer Length"),
                }
            },
            NetCommsErrorKind::OtherSource(_) => {
                match &self.message {
                    Some(message) => write!(f, "NetCommsError(Other Source):\n {}", message),
                    None => write!(f, "Other Source"),
                }
            },
        }
    }    
}

/// Struct to satisfy generics for Errors from this library.
#[derive(Debug)]
pub struct LibraryError;

impl std::fmt::Display for LibraryError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Library Error")
    }
}
impl std::error::Error for LibraryError {}

#[derive(Debug)]
pub struct OtherError;

impl std::fmt::Display for OtherError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Other Error")
    }
}
impl std::error::Error for OtherError {}

