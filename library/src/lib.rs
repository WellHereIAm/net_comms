//! Library for project of network communication using TCP protocol to send text messages and files.
//! Also implements convenient wrappers  of ToBuffer and FromBuffer for some types used in this library.


pub mod error_testing;

/// Module containing FromBuffer and ToBuffer traits.
pub mod buffer;
/// Module used to handle user input.
// This module will be probably completely refactored. Does not have documentation.
pub mod command;
/// Will be used for custom errors in the library.
// USE THIS FOR ALL CUSTOM ERRORS IN THE LIBRARY.
pub mod error;
/// Module used to handle Message.
pub mod message;
/// Module used to handle packets.
pub mod packet;
/// Module used to handle request like messages.
// This module will be probably completely refactored or even deleted. Does not have documentation.
pub mod request;
/// Module used to handle user.
// This module will be probably completely refactored. Does not have documentation.
pub mod user;
/// Shared constant and static variables for both server and client.
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

/// Wrappers of ToBuffer and FromBuffer around some types that are used inside this library.
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
                NetCommsErrorKind::InvalidBufferLength,
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
                NetCommsErrorKind::InvalidBufferLength,
                Some("Implementation from_buff for DateTime<Utc> requires buffer of length of at least 8 bytes.".to_string())))
        }
        let naive_datetime = NaiveDateTime::from_timestamp(usize::from_buff(buff)? as i64, 0);

        Ok(DateTime::from_utc(naive_datetime, Utc))  
    }   
    
}

