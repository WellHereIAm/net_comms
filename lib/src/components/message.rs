use crate::{Packet, PacketType};



pub enum MessageKind {
    Empty,
    Request,
    Text,
    File,
    Unknown,
} 

pub struct Message<T: PacketType> {
    contents: Vec<Packet<T>>
}

impl<T: PacketType> Message<T> {

    pub fn new() -> Self {
        todo!()
    }
}
