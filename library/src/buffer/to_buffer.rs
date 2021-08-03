/// Implementors of this trait can use its to_buff method
/// to transform itself to buffer.
pub trait ToBuffer {    

    /// This takes an ownership of self.
    fn to_buff(self) -> Vec<u8>;
}