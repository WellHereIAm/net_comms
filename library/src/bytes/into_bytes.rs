use chrono::{DateTime, Utc};

use crate::bytes::Bytes;

/// Allows all implementors perform conversion from `Self` into [Bytes]. 
pub trait IntoBytes {

    /// Performs a conversion from `Self` into [Bytes].
    fn into_bytes(self) -> Bytes;

    /// Uses [into_bytes](IntoBytes::into_bytes) to convert `Self` into [Vec] of [u8].
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