pub mod settings;

mod components {
    pub mod packet;
    pub mod message;
    pub mod request;
    pub mod user;
    pub mod command;
    pub mod buffer;

    pub use packet::*;
    pub use message::*;
    pub use request::*;
    pub use user::*;
    pub use command::*;
    pub use buffer::*;
}

pub use crate::components::*;
pub use crate::settings::*;


use chrono::{DateTime, NaiveDateTime, Utc};


// Wrappers around some types that are used inside packets.
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }   
    
}
