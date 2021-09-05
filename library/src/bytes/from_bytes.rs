use chrono::{DateTime, NaiveDateTime, Utc};

use crate::error::{NetCommsError, NetCommsErrorKind};

use crate::bytes::Bytes;

/// Allows all implementors perform conversion from [Bytes] and [slice] of [u8] into `Self`. 
pub trait FromBytes {

    /// Converts [`bytes`](Bytes) into `Self`.
    ///
    /// # Errors
    /// * This operation can fail if given `bytes` are not valid data to create `Self`
    fn from_bytes(bytes: Bytes) -> Result<Self, NetCommsError>
    where
        Self: Sized;
    
    /// Converts `buff` into `Self`.
    ///
    /// # Errors
    /// * This operation can fail if given `bytes` are not valid data to create `Self`
    fn from_buff(buff: &[u8]) -> Result<Self, NetCommsError>
    where
        Self: Sized;
}

impl FromBytes for usize {

    fn from_bytes(bytes: Bytes) -> Result<usize, NetCommsError> {

        // Check if buffer has valid length(at least 8).
        if None == bytes.get(7) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation FromBytes::from_bytes for usize requires bytes of length of at least 8 bytes.".to_string())))
        }

        let mut arr = [0_u8; 8];
        for (index, value) in bytes.into_iter().enumerate() {
            arr[index] = value;
        }
        Ok(usize::from_be_bytes(arr))
    }

    fn from_buff(buff: &[u8]) -> Result<Self, NetCommsError>
    where
        Self: Sized {
        
        // Check if buffer has valid length(at least 8).
        if None == buff.get(7) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation FromBytes::from_buff for usize requires buffer of length of at least 8 bytes.".to_string())))
        }

        let mut arr = [0_u8; 8];
        for (index, value) in buff.into_iter().enumerate() {
            arr[index] = *value;
        }
        Ok(usize::from_be_bytes(arr))
    }
}

impl FromBytes for u16 {

    fn from_bytes(bytes: Bytes) -> Result<u16, NetCommsError> {

        // Check if buffer has valid length(at least 2).
        if None == bytes.get(1) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation FromBytes::from_bytes for u16 requires bytes of length of at least 2 bytes.".to_string())))
        }

        let mut arr = [0_u8; 2];
        for (index, value) in bytes.into_iter().enumerate() {
            arr[index] = value;
        }
        Ok(u16::from_be_bytes(arr))
    }

    fn from_buff(buff: &[u8]) -> Result<Self, NetCommsError>
    where
        Self: Sized {
        
        // Check if buffer has valid length(at least 2).
        if None == buff.get(1) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation FromBytes::from_buff for u16 requires buffer of length of at least 2 bytes.".to_string())))
        }

        let mut arr = [0_u8; 2];
        for (index, value) in buff.into_iter().enumerate() {
            arr[index] = *value;
        }
        Ok(u16::from_be_bytes(arr))
    }
}

impl FromBytes for u32 {

    fn from_bytes(bytes: Bytes) -> Result<u32, NetCommsError> {

        // Check if buffer has valid length(at least 4).
        if None == bytes.get(3) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation FromBytes::from_bytes for usize requires bytes of length of at least 4 bytes.".to_string())))
        }

        let mut arr = [0_u8; 4];
        for (index, value) in bytes.into_iter().enumerate() {
            arr[index] = value;
        }
        Ok(u32::from_be_bytes(arr))
    }

    fn from_buff(buff: &[u8]) -> Result<u32, NetCommsError> {

       // Check if buffer has valid length(at least 4).
       if None == buff.get(3) {
        return Err(NetCommsError::new(
            NetCommsErrorKind::InvalidBufferSize,
            Some("Implementation FromBytes::from_buff for usize requires buffer of length of at least 4 bytes.".to_string())))
        }

        let mut arr = [0_u8; 4];
        for (index, value) in buff.into_iter().enumerate() {
            arr[index] = *value;
        }
        Ok(u32::from_be_bytes(arr)) 
    }
}

impl FromBytes for String {

    
    fn from_bytes(bytes: Bytes) -> Result<Self, NetCommsError>
    where
        Self: Sized {

        let string = String::from_utf8_lossy(bytes.as_slice()).into_owned();
        
        Ok(string)
    }
    
    fn from_buff(buff: &[u8]) -> Result<String, NetCommsError> {

        let string = String::from_utf8_lossy(buff).into_owned();
        
        Ok(string)
    }
}

impl FromBytes for DateTime<Utc> {

    fn from_bytes(bytes: Bytes) -> Result<DateTime<Utc>, NetCommsError> {

        // Check if buffer has valid length(at least 8).
        if None == bytes.get(7) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation FromBytes::from_bytes for DateTime<Utc> requires buffer of length of at least 8 bytes.".to_string())))
        }
        let naive_datetime = NaiveDateTime::from_timestamp(usize::from_bytes(bytes)? as i64, 0);

        Ok(DateTime::from_utc(naive_datetime, Utc))  
    }

    fn from_buff(buff: &[u8]) -> Result<Self, NetCommsError>
    where
        Self: Sized {

        if let None = buff.get(7) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation FromBytes::from_buff for DateTime<Utc> requires buffer of length of at least 8 bytes.".to_string())))
        }

        let naive_datetime = NaiveDateTime::from_timestamp(usize::from_buff(buff)? as i64, 0);
        
        Ok(DateTime::from_utc(naive_datetime, Utc))  
    }  
}