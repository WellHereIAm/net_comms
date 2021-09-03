use std::net::TcpStream;
use std::path::PathBuf;

use crate::error::NetCommsError;
use crate::ron::{FromRon, IntoRon};

pub trait MetaDataType<'a>: Default + Clone + FromRon<'a> + IntoRon
where
    Self: Sized {
    fn send(self, stream: &mut TcpStream) -> Result<Self, NetCommsError>;
    fn receive(stream: &mut TcpStream, location: Option<PathBuf>) -> Result<Self, NetCommsError>;
}