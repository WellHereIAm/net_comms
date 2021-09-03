use serde::{Serialize, Deserialize};

use library::bytes::{Bytes, FromBytes, IntoBytes};
use library::message::MessageKindType;
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

impl Default for MessageKind {

    fn default() -> Self {
        MessageKind::Empty
    }
}

impl MessageKindType<'_> for MessageKind {}

impl FromRon<'_> for MessageKind {}
impl IntoRon for MessageKind {}

impl IntoBytes for MessageKind {

    fn into_bytes(self) -> Bytes {

        let msg_kind = match self {
            MessageKind::Empty => [0_u8, 0_u8],
            MessageKind::Request => [1_u8, 0_u8],
            MessageKind::Text => [2_u8, 0_u8],
            MessageKind::File => [3_u8, 0_u8],
            MessageKind::SeverReply => [4_u8, 0_u8],
            MessageKind::Unknown => [255_u8, 0_u8],
        };

        Bytes::from_arr(msg_kind)
    }
}

impl FromBytes for MessageKind {

    fn from_bytes(bytes: Bytes) -> Result<Self, NetCommsError>
    where
            Self: Sized {
        
        // Check if buffer has valid length(at least 2, anything beyond that is discarded.).
        if None == bytes.get(1) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation from_buff for MessageKind requires buffer of length of at least 2 bytes.".to_string())))
        }

        let msg_kind = match bytes[0] {
            0 => MessageKind::Empty,
            1 => MessageKind::Request,
            2 => MessageKind::Text,
            3 => MessageKind::File,
            4 => MessageKind::SeverReply,
            _ => MessageKind::Unknown,            
        }; 
        
        Ok(msg_kind)
    }

    fn from_buff(buff: &[u8]) -> Result<Self, NetCommsError>
    where
            Self: Sized {
        
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