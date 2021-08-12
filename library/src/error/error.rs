use backtrace::Backtrace;

use crate::error::NetCommsErrorKind;

/// Error struct for this library.
// #[derive(Debug)]
pub struct NetCommsError {
    pub kind: NetCommsErrorKind,
    pub message: Option<String>,
    pub backtrace: Backtrace,
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

    pub fn new(kind: NetCommsErrorKind, message: Option<String>) -> Self {
        
        let backtrace = Backtrace::new(); 

        NetCommsError {
            kind,
            message,
            backtrace,
        }
    }

    fn output(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self.kind {
            NetCommsErrorKind::WrongCommand => {
                match &self.message {
                    Some(message) => write!(f, "
                    \n
                    NetCommsError(Wrong Command):\n
                    {}\n
                    source:\n
                    {:?}", message, self.backtrace),
                    None => write!(f, " Wrong Command"),
                }
            },
            NetCommsErrorKind::InvalidPacketKind => {
                match &self.message {
                    Some(message) => write!(f, "
                    \n
                    NetCommsError(Invalid Packet Kind):\n
                    {}\n
                    source:\n
                    {:?}", message, self.backtrace),
                    None => write!(f, " Invalid Packet Kind"),
                }
            },
            NetCommsErrorKind::SerializingFailed => {
                match &self.message {
                    Some(message) => write!(f, "
                    \n
                    NetCommsError(Serializing Failed):\n
                    {}\n
                    source:\n
                    {:?}", message, self.backtrace),
                    None => write!(f, " Serializing Failed"),
                }
            },
            NetCommsErrorKind::DeserializingFailed => {
                match &self.message {
                    Some(message) => write!(f, "\n
                    NetCommsError(Deserializing Failed):\n
                    {}\n
                    source:\n
                    {:?}", message, self.backtrace),
                    None => write!(f, " Deserializing Failed"),
                }
            },
            NetCommsErrorKind::InvalidBufferLength => {
                match &self.message {
                    Some(message) => write!(f, "
                    \n
                    NetCommsError(Invalid Buffer Length):\n
                    {}\n
                    source:\n
                    {:?}", message, self.backtrace),
                    None => write!(f, " Invalid Buffer Length"),
                }
            },
            NetCommsErrorKind::WritingToStreamFailed => {
                match &self.message {
                    Some(message) => write!(f, "
                    \n
                    NetCommsError(Writing To Stream Failed):\n
                    {}\n
                    source:\n
                    {:?}", message, self.backtrace),
                    None => write!(f, " Writing To Stream Failed"),
                }
            },
            NetCommsErrorKind::ReadingFromStreamFailed => {
                match &self.message {
                    Some(message) => write!(f, "
                    \n
                    NetCommsError(Reading From Stream Failed):\n
                    {}\n
                    source:\n
                    {:?}", message, self.backtrace),
                    None => write!(f, " Reading From Stream Failed"),
                }
            },
            NetCommsErrorKind::OpeningFileFailed => {
                match &self.message {
                    Some(message) => write!(f, "
                    \n
                    NetCommsError(Opening File Failed):\n
                    {}\n
                    source:\n
                    {:?}", message, self.backtrace),
                    None => write!(f, " Opening File Failed"),
                }
            },
            NetCommsErrorKind::WritingToFileFailed => {
                match &self.message {
                    Some(message) => write!(f, "
                    \n
                    NetCommsError(Writing To File Failed):\n
                    {}\n
                    source:\n
                    {:?}", message, self.backtrace),
                    None => write!(f, " Writing To File Failed"),
                }
            },
            NetCommsErrorKind::ReadingFromFileFailed => {
                match &self.message {
                    Some(message) => write!(f, "
                    \n
                    NetCommsError(Reading From File Failed):\n
                    {}\n
                    source:\n
                    {:?}", message, self.backtrace),
                    None => write!(f, " Reading From File Failed"),
                }
            }
            NetCommsErrorKind::IncompleteMetaData => {
                match &self.message {
                    Some(message) => write!(f, "
                    \n
                    NetCommsError(Incomplete MetaData):\n
                    {}\n
                    source:\n
                    {:?}", message, self.backtrace),
                    None => write!(f, " Incomplete MetaData"),
                }
            }
            NetCommsErrorKind::OtherSource(_) => {
                match &self.message {
                    Some(message) => write!(f, "
                    \n
                    NetCommsError(Other Source):\n
                    {}\n
                    source:\n
                    {:?}", message, self.backtrace),
                    None => write!(f, " Other Source"),
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

