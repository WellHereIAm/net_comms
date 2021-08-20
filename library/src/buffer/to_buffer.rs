use crate::error::NetCommsError;

/// Implementors of this trait can use its `to_buff` method to transform a self to a buffer -- [`Vec<u8>`](Vec).
pub trait ToBuffer {    

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
    /// * Usual cause of error inside this method is failed serialization of given type to RON, in which case 
    /// [NetCommsError](crate::error::NetCommsError) with kind: [SerializingFailed](crate::error::NetCommsErrorKind::SerializingFailed) is returned.
    /// * This can also return other [NetCommsError].
    fn to_buff(self) -> Result<Vec<u8>, NetCommsError>;
}