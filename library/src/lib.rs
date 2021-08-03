//! Library for project of network communication using TCP protocol to send text messages and files.
//! Also implements convenient wrappers  of ToBuffer and FromBuffer for some types used in this library.

/// Module containing FromBuffer and ToBuffer traits.
pub mod buffer;
/// Module containing MessageKind and Message.
pub mod message;
/// Module containing Packet and all it´s parts.
pub mod packet;

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

