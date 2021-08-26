use chrono::{DateTime, NaiveDateTime, Utc};

use crate::error::{NetCommsError, NetCommsErrorKind};

/// Implementors of this trait can use its `from_buff` method to transform a buffer to type of implementor.
pub trait FromBuffer {
    
    /// This takes an ownership of buff and transforms it to the implementor type.
    ///
    /// This function should never panic.
    /// # Arguments
    ///
    /// `buff` -- Vector containing bytes that should be converted to given type.
    ///
    /// # Examples
    /// 
    /// ```
    /// // This works because FromBuffer is implemented for usize in this library.
    /// assert_eq!(5, usize::from_buff(vec![0, 0, 0, 0, 0, 0, 0, 5]).unwrap()); 
    /// ```
    /// # Errors
    ///
    /// * This should return [NetCommsError](crate::error::NetCommsError) with [InvalidBufferSize](crate::error::NetCommsErrorKind::InvalidBufferSize)
    /// as a kind, if the given buffer does does not have proper length to be converted to implementor type.
    /// * Other cause of error inside this method is failed deserialization of [RON](ron) to given type, in which case 
    /// [NetCommsError](crate::error::NetCommsError) with kind: [DeserializingFailed](crate::error::NetCommsErrorKind::DeserializingFailed) is returned.
    /// * This can also return other [NetCommsError].
    fn from_buff(buff: Vec<u8>) -> Result<Self, NetCommsError>
    where
        Self: Sized;
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

impl FromBuffer for u16 {

    fn from_buff(buff: Vec<u8>) -> Result<u16, NetCommsError> {

        // Check if buffer has valid length(at least 2).
        if None == buff.get(1) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation from_buff for usize requires buffer of length of at least 2 bytes.".to_string())))
        }

        let mut arr = [0_u8; 2];
        for (index, value) in buff.into_iter().enumerate() {
            if index == 2 {
                break;
            }
            arr[index] = value;
        }
        Ok(u16::from_be_bytes(arr))
    }
}

impl FromBuffer for u32 {

    fn from_buff(buff: Vec<u8>) -> Result<u32, NetCommsError> {

        // Check if buffer has valid length(at least 4).
        if None == buff.get(3) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation from_buff for usize requires buffer of length of at least 4 bytes.".to_string())))
        }

        let mut arr = [0_u8; 4];
        for (index, value) in buff.into_iter().enumerate() {
            arr[index] = value;
        }
        Ok(u32::from_be_bytes(arr))
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