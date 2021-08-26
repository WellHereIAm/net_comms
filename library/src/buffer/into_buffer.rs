use chrono::{DateTime, Utc};

use crate::error::NetCommsError;

/// Implementors of this trait can use its `into_buff` method to transform a self to a buffer -- [`Vec<u8>`](Vec).
pub trait IntoBuffer {    

    /// This takes an ownership of self and transforms it to buffer.
    ///
    /// This function should never panic.
    ///
    /// # Examples
    /// 
    /// ```
    /// // This works because ToBuffer is implemented for usize in this library.
    /// assert_eq!(5.to_buff().unwrap(), vec![0, 0, 0, 0, 0, 0, 0, 5]); 
    /// ```
    /// # Errors
    ///
    /// * Usual cause of error inside this method is failed serialization of given type to [RON](ron), in which case 
    /// [NetCommsError](crate::error::NetCommsError) with kind: [SerializingFailed](crate::error::NetCommsErrorKind::SerializingFailed) is returned.
    /// * This can also return other [NetCommsError].
    fn into_buff(self) -> Result<Vec<u8>, NetCommsError>;
}

impl IntoBuffer for usize {
    fn into_buff(self) -> Result<Vec<u8>, NetCommsError> {
        
        Ok(self.to_be_bytes().to_vec())
    }
}

impl IntoBuffer for u16 {
    fn into_buff(self) -> Result<Vec<u8>, NetCommsError> {
        
        Ok(self.to_be_bytes().to_vec())
    }
}

impl IntoBuffer for u32 {
    fn into_buff(self) -> Result<Vec<u8>, NetCommsError> {
        
        Ok(self.to_be_bytes().to_vec())
    }
}

impl IntoBuffer for String {

    fn into_buff(self) -> Result<Vec<u8>, NetCommsError> {
        Ok(self.as_bytes().to_vec())
    }
}

impl IntoBuffer for DateTime<Utc> {

    fn into_buff(self) -> Result<Vec<u8>, NetCommsError> {
        (self.timestamp() as usize).into_buff()
    }
}