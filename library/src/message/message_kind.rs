use serde::{Serialize, Deserialize};

use crate::buffer::{ToBuffer, FromBuffer};


/// Holds a kind of every Message to be sent or received.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageKind {
    Empty,
    Request,
    Text,
    File,
    Unknown,
}

impl ToBuffer for MessageKind {

    /// This takes an ownership of self.
    fn to_buff(self) -> Vec<u8> {

        let msg_kind = match self {
            MessageKind::Empty => [0_u8, 0_u8],
            MessageKind::Request => [1_u8, 0_u8],
            MessageKind::Text => [2_u8, 0_u8],
            MessageKind::File => [3_u8, 0_u8],
            MessageKind::Unknown => [255_u8, 0_u8],
        };
        msg_kind.to_vec()
    }    
}

impl FromBuffer for MessageKind {
    
    fn from_buff(buff: Vec<u8>) -> Self {

        let msg_kind = match buff[0] {
            0 => MessageKind::Empty,
            1 => MessageKind::Request,
            2 => MessageKind::Text,
            3 => MessageKind::File,
            _ => MessageKind::Unknown,            
        }; 
        
        msg_kind
    }
}