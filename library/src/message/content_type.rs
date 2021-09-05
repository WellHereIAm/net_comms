use std::fmt::{Debug, Display};
use std::net::TcpStream;
use std::path::PathBuf;

use crate::error::NetCommsError;
use crate::packet::Packet;
use crate::ron::{FromRon, ToRon};
use crate::message::MetaDataType;

/// Trait that has to be implemented by every struct than will be used as `content` inside a [Message](super::Message).
pub trait ContentType<'a, M, C>:
    Default + Clone + Display + Debug + FromRon<'a> + ToRon 
where
    M: MetaDataType<'a>,
    C: ContentType<'a, M, C>, {
    
    /// Send method used to send `content` inside [Message::send](super::Message::send).
    ///
    /// Defines how `content` is send.
    ///
    /// * `metadata` -- metadata of [Message](super::Message) that is being sent. 
    fn send(self, stream: &mut TcpStream, metadata: M) -> Result<(), NetCommsError>;

    /// Receive method used to receive `content` inside [Message::receive](super::Message::receive).
    ///
    /// Defines how `content` is received.
    ///
    /// # Fields
    ///
    /// * `stream` -- [TcpStream] on which is [Message](super::Message) being received.
    /// * `metadata` -- a reference for already received `metadata`.
    // This may be deleted.
    /// * `location` -- optional location on the disk, that can be used to save some data from message.
    fn receive(stream: &mut TcpStream,
               metadata: &M,
               location: Option<PathBuf>) -> Result<(Self, Packet), NetCommsError>;
}