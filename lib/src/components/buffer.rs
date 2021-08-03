pub trait ToBuffer {    

    /// !!!
    /// This method gets an ownership of self.
    fn to_buff(self) -> Vec<u8>;
}

pub trait FromBuffer {
    
    fn from_buff(buff: Vec<u8>) -> Self;
}

// Maybe at some point change this to std::io::Write as it does exactly that.
pub fn write_to_buff(buff: &mut Vec<u8>, slice: &[u8]) {
        
    for byte in slice {
        buff.push(*byte);
    }
}

//fn buff_to_field<const N: usize>(buff: Vec<u8>, range: Range<usize>) -> [u8; N] {
//
//    let mut field = [0_u8; N];
//
//    for (index, value) in buff[range].into_iter().enumerate() {
//        field[index] = *value;
//    }
//
//    field
//}