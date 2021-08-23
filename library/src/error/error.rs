use backtrace::Backtrace;
use indoc::indoc;

use crate::error::NetCommsErrorKind;


/// Used as an [Error type](std::error::Error) throughout this library.
///
/// # Fields
/// 
/// `kind` -- [kind](NetCommsErrorKind) of error.
///
/// `message` -- optional additional information about the error.
///
/// `backtrace` -- stack backtrace of error, created via [backtrace].
pub struct NetCommsError {
    kind: NetCommsErrorKind,
    message: Option<String>,
    backtrace: Backtrace,
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

    /// Method used to construct a new [NetCommsError].
    ///
    /// Also creates a backtrace.
    ///
    /// # Arguments
    /// `kind` -- [kind](NetCommsErrorKind) of error.
    ///
    /// `message` -- optional additional information about the error.
    ///
    /// # Examples
    /// 
    /// ```
    /// use std::fs::File;
    ///
    /// match File::open("does_not_exist.rs") {
    ///     Ok(file) => {
    ///     // Do something with the file.
    ///     } 
    ///     Err(e) => {
    ///         return NetCommsError::new(
    ///                    NetCommsErrorKind::OpeningFileFailed,
    ///                    Some("Failed to open a file.")
    ///                )
    ///     }   
    /// }
    /// ```
    pub fn new(kind: NetCommsErrorKind, message: Option<String>) -> Self {
        
        let backtrace = Backtrace::new(); 

        NetCommsError {
            kind,
            message,
            backtrace,
        }
    }

    /// Method used to format the output of the error.
    fn output(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self.kind {
            NetCommsErrorKind::UnknownCommand => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Unknown Command):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Unknown Command)"),
                }
            },
            NetCommsErrorKind::InvalidCommand => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Invalid Command):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Invalid Command)"),
                }
            }
            NetCommsErrorKind::InvalidPacketKind => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Invalid Packet Kind):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Invalid Packet Kind)"),
                }
            },
            NetCommsErrorKind::SerializingFailed => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Serializing Failed):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Serializing Failed)"),
                }
            },
            NetCommsErrorKind::DeserializingFailed => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Deserializing Failed):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Deserializing Failed)"),
                }
            },
            NetCommsErrorKind::InvalidBufferSize => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Invalid Buffer Size):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Invalid Buffer Size)"),
                }
            },
            NetCommsErrorKind::WritingToStreamFailed => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Writing To Stream Failed):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Writing To Stream Failed)"),
                }
            },
            NetCommsErrorKind::ReadingFromStreamFailed => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Reading From Stream Failed):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Reading From Stream Failed)"),
                }
            },
            NetCommsErrorKind::CreatingDirFailed => {
                match &self.message {
                    Some(message) => write!(f, 
                    indoc! {
                    "
                    \n
                    NetCommsError(Creating Dir Failed):\n
                    {}\n
                    source:\n
                    {:?}
                    "
                    }
                    , message, self.backtrace),
                    None => write!(f, "NetCommsError(Creating Dir Failed)"),
                }
            },
            NetCommsErrorKind::CreatingFileFailed => {
                match &self.message {
                    Some(message) => write!(f, 
                    indoc! {
                    "
                    \n
                    NetCommsError(Creating File Failed):\n
                    {}\n
                    source:\n
                    {:?}
                    "
                    }
                    , message, self.backtrace),
                    None => write!(f, "NetCommsError(Creating File Failed)"),
                }
            },
            NetCommsErrorKind::OpeningFileFailed => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Opening File Failed):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Opening File Failed)"),
                }
            },
            NetCommsErrorKind::WritingToFileFailed => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Writing To File Failed):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Writing To File Failed)"),
                }
            },
            NetCommsErrorKind::ReadingFromFileFailed => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Reading From File Failed):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Reading From File Failed)"),
                }
            }
            NetCommsErrorKind::IncompleteMetaData => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Incomplete MetaData):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Incomplete MetaData)"),
                }
            }
            NetCommsErrorKind::UnknownMessageKind => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Unknown Message Kind):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Unknown Message Kind)"),
                }
            }
            NetCommsErrorKind::OtherSource(_) => {
                match &self.message {
                    Some(message) => write!(f, 
                        indoc! {
                        "
                        \n
                        NetCommsError(Other Source):\n
                        {}\n
                        source:\n
                        {:?}
                        "
                        }
                        , message, self.backtrace),
                    None => write!(f, "NetCommsError(Other Source)"),
                }
            },
        }
    }    
}


# [test]
fn error_format() {
    let error = NetCommsError::new(
        NetCommsErrorKind::UnknownCommand,
        None);
    println!("{}", error);
}