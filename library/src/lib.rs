//! Library for project of network communication using TCP protocol to send text messages and files.
//! Also implements convenient wrappers  of ToBuffer and FromBuffer for some types used in this library.


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
/// Shared configurations for both server and client.
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

// Wrappers of ToBuffer and FromBuffer around some types that are used inside this library.
impl ToBuffer for usize {

    fn to_buff(self) -> Vec<u8> {
        
        self.to_be_bytes().to_vec()
    }
}

impl FromBuffer for usize {

    fn from_buff(buff: Vec<u8>) -> Self {

        let mut arr = [0_u8; 8];
        for (index, value) in buff.into_iter().enumerate() {
            arr[index] = value;
        }
        usize::from_be_bytes(arr)
    }
}

impl ToBuffer for String {

    fn to_buff(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl FromBuffer for String {
    
    fn from_buff(buff: Vec<u8>) -> Self {
        String::from_utf8_lossy(&buff).to_string()
    }
}

impl ToBuffer for DateTime<Utc> {

    fn to_buff(self) -> Vec<u8> {
        (self.timestamp() as usize).to_buff()
    }
}

impl FromBuffer for DateTime<Utc> {

    fn from_buff(buff: Vec<u8>) -> Self {
        let naive_datetime = NaiveDateTime::from_timestamp(usize::from_buff(buff) as i64, 0);

        DateTime::from_utc(naive_datetime, Utc)  
    }   
    
}

