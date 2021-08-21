//! Framework that eases network communication using TCP protocol.
//! 
//! This is only a library which declared ways how are data structured inside the buffer containing sent or received data.
//! This framework is build on top of [std::net].


/// Module containing [FromBuffer] and [ToBuffer] traits.
///
/// Those traits are used throughout this library as they provide necessary functionality for given type to convert it to or from buffer,
/// which inside this library is always [Vec] of [u8].
pub mod buffer;

/// Module used to get and process user input through commands.
pub mod command;

/// [Error type](std::error::Error) for this library.
pub mod error;

/// Module containing [Message](message::Message) and other struct that are used inside it or with it.
pub mod message;

/// Module containing [Packet](packet::Packet) and other struct that are used inside it or with it.
pub mod packet;

/// Module containing [Request](request::Request), which is used to send requests from client to server.
pub mod request;

/// Module containing structs that are used for user identification.
pub mod user;

/// Shared constant and static variables used throughout this library.
pub mod config;

/// Module to simplify development, so I can use use library::prelude::*, most likely will be deleted later.
pub mod prelude {
    pub use crate::command::{self, *};
    pub use crate::buffer::{self, *};
    pub use crate::error::{self, *};
    pub use crate::message::{self, *};
    pub use crate::packet::{self, *};
    pub use crate::request::{self, *};
    pub use crate::user::{self, *};
    pub use crate::config::{self, *};
}

use chrono::{DateTime, NaiveDateTime, Utc};

use buffer::{ToBuffer, FromBuffer};
use crate::error::{NetCommsError, NetCommsErrorKind};


impl ToBuffer for usize {
    fn to_buff(self) -> Result<Vec<u8>, NetCommsError> {
        
        Ok(self.to_be_bytes().to_vec())
    }
}

impl FromBuffer for usize {

    fn from_buff(buff: Vec<u8>) -> Result<usize, NetCommsError> {

        // Check if buffer has valid length(at least 8).
        if None == buff.get(7) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation from_buff for usize requires buffer of length of at least 8 bytes.".to_string())))
        }

        let mut arr = [0_u8; 8];
        for (index, value) in buff.into_iter().enumerate() {
            arr[index] = value;
        }
        Ok(usize::from_be_bytes(arr))
    }
}

impl ToBuffer for String {

    fn to_buff(self) -> Result<Vec<u8>, NetCommsError> {
        Ok(self.as_bytes().to_vec())
    }
}

impl FromBuffer for String {
    
    fn from_buff(buff: Vec<u8>) -> Result<String, NetCommsError> {
        match String::from_utf8(buff) {
            Ok(string) => Ok(string),
            Err(e) => Err(NetCommsError::new(
                NetCommsErrorKind::OtherSource(Box::new(e)),
                None))
        }
    }
}

impl ToBuffer for DateTime<Utc> {

    fn to_buff(self) -> Result<Vec<u8>, NetCommsError> {
        (self.timestamp() as usize).to_buff()
    }
}

impl FromBuffer for DateTime<Utc> {

    fn from_buff(buff: Vec<u8>) -> Result<DateTime<Utc>, NetCommsError> {

        // Check if buffer has valid length(at least 8).
        if None == buff.get(7) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation from_buff for DateTime<Utc> requires buffer of length of at least 8 bytes.".to_string())))
        }
        let naive_datetime = NaiveDateTime::from_timestamp(usize::from_buff(buff)? as i64, 0);

        Ok(DateTime::from_utc(naive_datetime, Utc))  
    }  
}

