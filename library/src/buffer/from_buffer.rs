use crate::error::NetCommsError;

/// Implementors of this trait can use its from_buff method
/// to transform a buffer to given Type of implementor.
pub trait FromBuffer {
    
    /// This takes an ownership of buff.
    /// Every implementation should check if the buffer has valid length and return NetCommsError::InvalidBufferSize if does not.
    fn from_buff(buff: Vec<u8>) -> Result<Self, NetCommsError> where Self: Sized;
}