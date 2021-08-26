use serde::{Serialize, Deserialize};

use library::message::MessageKindType;
use library::buffer::{IntoBuffer, FromBuffer};
use library::error::{NetCommsError, NetCommsErrorKind};
use library::ron::{FromRon, IntoRon};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageKind {
    Empty,
    Request,
    Text,
    File,
    SeverReply,
    Unknown,
}

impl MessageKindType for MessageKind {}

impl FromRon for MessageKind {}
impl IntoRon for MessageKind {}

impl IntoBuffer for MessageKind {

    fn into_buff(self) -> Result<Vec<u8>, NetCommsError> {

        let msg_kind = match self {
            MessageKind::Empty => [0_u8, 0_u8],
            MessageKind::Request => [1_u8, 0_u8],
            MessageKind::Text => [2_u8, 0_u8],
            MessageKind::File => [3_u8, 0_u8],
            MessageKind::SeverReply => [4_u8, 0_u8],
            MessageKind::Unknown => [255_u8, 0_u8],
        };
        Ok(msg_kind.to_vec())
    }    
}

impl FromBuffer for MessageKind {
    
    fn from_buff(buff: Vec<u8>) -> Result<MessageKind, NetCommsError> {

        // Check if buffer has valid length(at least 2, anything beyond that is discarded.).
        if None == buff.get(1) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation from_buff for MessageKind requires buffer of length of at least 2 bytes.".to_string())))
        }

        let msg_kind = match buff[0] {
            0 => MessageKind::Empty,
            1 => MessageKind::Request,
            2 => MessageKind::Text,
            3 => MessageKind::File,
            4 => MessageKind::SeverReply,
            _ => MessageKind::Unknown,            
        }; 
        
        Ok(msg_kind)
    }
}