use serde::{Serialize, Deserialize};

use crate::buffer::{ToBuffer, FromBuffer};
use crate::error::{NetCommsError, NetCommsErrorKind};

/// Holds a kind of [Message](super::Message).
///
/// When sent it is stored inside [Message](super::Message) `metadata`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageKind {
    /// Empty struct, usually used only while creating new [Message](super::Message) using [Message::new](super::Message::new).
    Empty,
    /// Request from client.
    Request,
    /// Text message sent from one user to another.
    Text,
    /// File message sent from one user to another.
    File,
    /// Reply from server to client.
    SeverReply,
    /// Used in case if [Message](super::Message) fails to recognize a [MessageKind].
    Unknown,
}

impl ToBuffer for MessageKind {

    /// This takes an ownership of self.
    fn to_buff(self) -> Result<Vec<u8>, NetCommsError> {

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