use crate::error::NetCommsError;

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
    /// * Other cause of error inside this method is failed deserialization of RON to given type, in which case 
    /// [NetCommsError](crate::error::NetCommsError) with kind: [DeserializingFailed](crate::error::NetCommsErrorKind::DeserializingFailed) is returned.
    /// * This can also return other [NetCommsError].
    fn from_buff(buff: Vec<u8>) -> Result<Self, NetCommsError>
    where
        Self: Sized;
}