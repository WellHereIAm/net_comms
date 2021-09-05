use std::net::TcpStream;
use std::path::PathBuf;

use crate::error::NetCommsError;
use crate::ron::{FromRon, ToRon};

/// Trait that has to be implemented by every struct than will be used as `metadata` inside a [Message](super::Message).
pub trait MetaDataType<'a>: Default + Clone + FromRon<'a> + ToRon
where
    Self: Sized {

    /// Send method used to send `metadata` inside [Message::send](super::Message::send).
    ///
    /// Defines how `metadata` are send.  
    fn send(self, stream: &mut TcpStream) -> Result<Self, NetCommsError>;
    
    /// Receive method used to receive `metadata` inside [Message::receive](super::Message::receive).
    ///
    ///
    /// Defines how `content` is received.
    ///
    /// * `location` -- optional location on the disk, that can be used to save some data from message.
    fn receive(stream: &mut TcpStream, location: Option<PathBuf>) -> Result<Self, NetCommsError>;
}