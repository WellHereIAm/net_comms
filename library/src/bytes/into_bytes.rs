use crate::bytes::Bytes;

pub trait IntoBytes {
    fn into_bytes(self) -> Bytes;
}

impl IntoBytes for Vec<u8> {
    
    fn into_bytes(self) -> Bytes {
        Bytes::from_vec(self)
    }
}

impl IntoBytes for String {
    
    fn into_bytes(self) -> Bytes {
        Bytes::from_vec(self.as_bytes().to_vec())
    }
}