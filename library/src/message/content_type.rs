use std::fmt::{Debug, Display};
use std::net::TcpStream;
use std::path::PathBuf;

use crate::error::NetCommsError;
use crate::packet::Packet;
use crate::ron::{FromRon, IntoRon};
use crate::message::{Message, MetaDataType};

pub trait ContentType<'a, M, C>:
    Default + Clone + Display + Debug + FromRon<'a> + IntoRon 
where
    M: MetaDataType<'a>,
    C: ContentType<'a, M, C>, {
    
    fn send(self, stream: &mut TcpStream, metadata: M) -> Result<(), NetCommsError>;
    fn receive(stream: &mut TcpStream,
               message: &mut Message<M, C>,
               location: Option<PathBuf>) -> Result<(Self, Packet), NetCommsError>;
}