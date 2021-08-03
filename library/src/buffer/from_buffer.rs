/// Implementors of this trait can use its from_buff method
/// to transform a buffer to given Type of implementor.
pub trait FromBuffer {
    
    /// This takes an ownership of buff.
    fn from_buff(buff: Vec<u8>) -> Self;
}