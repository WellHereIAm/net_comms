#[derive(Debug)]
pub struct PacketKindError {}

impl std::fmt::Display for PacketKindError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PacketKindError")
    }    
}

impl std::error::Error for PacketKindError {}

#[derive(Debug)]
pub struct PacketError {}

impl std::fmt::Display for PacketError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PacketError")
    }    
}

impl std::error::Error for PacketError {}