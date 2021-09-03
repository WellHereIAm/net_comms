use serde::{Serialize, Deserialize};

use crate::bytes::{Bytes, FromBytes, IntoBytes};
// use crate::buffer::{IntoBuffer, FromBuffer};
use crate::error::{NetCommsError, NetCommsErrorKind};
use crate::ron::{IntoRon, FromRon};


/// Determines kind of [Packet](super::Packet).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacketKind {

    /// Empty packet usually used only for testing or like a placeholder.
    Empty,  

    /// Part of [MetaData](super::MetaData) encoded in [RON](ron) format.
    MetaData,  

    /// It is same as [MetaData], differing only in fact that this variant is used to sign the last part of [MetaData](super::MetaData).
    MetaDataEnd, 

    /// Content packet, content depends on [MessageKind](crate::message::MessageKind) which is described in metadata of related
    /// [Message](crate::message::Message)
    Content, 

    /// Used to signalize end of the [Message](crate::message::Message)
    End, 

    /// Used in case if [PacketKind] is not recognized.
    Unknown,
}

impl IntoRon for PacketKind {}
impl FromRon<'_> for PacketKind {}


impl FromBytes for PacketKind {
    
    fn from_bytes(bytes: Bytes) -> Result<Self, NetCommsError>
    where
            Self: Sized {

        // Check if buffer has valid length(at least 2).
        match bytes.get(1) {
            Some(_) => &bytes[0..2],
            None => return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation from_buff for PacketKind requires buffer of length of at least three bytes.".to_string())))
        };

        let kind = match bytes[0] {
            0 => PacketKind::Empty,
            1 => match bytes[1] {
                    0 => PacketKind::MetaData,
                    1 => PacketKind::MetaDataEnd,
                    _ => PacketKind::Unknown,              
            }
            2 => PacketKind::Content,
            3 => PacketKind::End,
            _ => PacketKind::Unknown,  
        };

        Ok(kind)          
    }

    fn from_buff(buff: &[u8]) -> Result<Self, NetCommsError>
    where
            Self: Sized {

        // Check if buffer has valid length(at least 2).
        match buff.get(1) {
            Some(_) => &buff[0..2],
            None => return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation from_buff for PacketKind requires buffer of length of at least three bytes.".to_string())))
        };

        let kind = match buff[0] {
            0 => PacketKind::Empty,
            1 => match buff[1] {
                    0 => PacketKind::MetaData,
                    1 => PacketKind::MetaDataEnd,
                    _ => PacketKind::Unknown,              
            }
            2 => PacketKind::Content,
            3 => PacketKind::End,
            _ => PacketKind::Unknown,  
        };

        Ok(kind)  
    }
}

impl IntoBytes for PacketKind {

    fn into_bytes(self) -> Bytes {
        
        let mut bytes = Bytes::new();

        match self {
            PacketKind::Empty => bytes.append(&mut Bytes::from_arr([0, 0])),
            PacketKind::MetaData => bytes.append(&mut Bytes::from_arr([1, 0])),
            PacketKind::MetaDataEnd => bytes.append(&mut Bytes::from_arr([1, 1])),
            PacketKind::Content => bytes.append(&mut Bytes::from_arr([2, 0])),
            PacketKind::End => bytes.append(&mut Bytes::from_arr([3, 0])),
            PacketKind::Unknown => bytes.append(&mut Bytes::from_arr([255, 0])),
        }

        bytes
    }
}
