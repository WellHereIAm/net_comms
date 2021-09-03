use chrono::{DateTime, Utc};

use crate::bytes::Bytes;

pub trait IntoBytes {

    /// Some test text
    fn into_bytes(self) -> Bytes;

    fn into_buff(self) -> Vec<u8> 
    where
        Self: Sized {
        self.into_bytes().into_vec()
    }
}

impl IntoBytes for Vec<u8> {
    
    fn into_bytes(self) -> Bytes {
        Bytes::from_vec(self)
    }
}

impl IntoBytes for usize {
    
    fn into_bytes(self) -> Bytes {
        Bytes::from_vec(self.to_be_bytes().to_vec())
    }
}

impl IntoBytes for u16 {
    
    fn into_bytes(self) -> Bytes {
        Bytes::from_vec(self.to_be_bytes().to_vec())
    }
}

impl IntoBytes for u32 {
    
    fn into_bytes(self) -> Bytes {
        Bytes::from_vec(self.to_be_bytes().to_vec())
    }
}

impl IntoBytes for String {
    
    fn into_bytes(self) -> Bytes {
        Bytes::from_vec(self.into_bytes())
    }
}

impl IntoBytes for DateTime<Utc> {
    
    fn into_bytes(self) -> Bytes {
        (self.timestamp() as usize).into_bytes()
    }
}