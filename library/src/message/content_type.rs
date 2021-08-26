use std::fmt::{Debug, Display};
use std::net::TcpStream;
use std::path::PathBuf;

use crate::error::NetCommsError;
use crate::packet::Packet;
use crate::ron::{FromRon, IntoRon};
use crate::message::{Message, MessageKindType, MetaDataType};

pub trait ContentType<'a, K, M, C>:
    Default + Clone + Display + Debug + FromRon<'a> + IntoRon 
where
    K: MessageKindType<'a>,
    M: MetaDataType<'a>,
    C: ContentType<'a, K, M, C>, {
    
    fn send(self, stream: &mut TcpStream, metadata: M) -> Result<(), NetCommsError>;
    fn receive(stream: &mut TcpStream,
               message: &mut Message<K, M, C>,
               location: Option<PathBuf>) -> Result<(Self, Packet), NetCommsError>;
}