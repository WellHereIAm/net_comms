use serde::{Serialize, Deserialize};

use crate::buffer::{IntoBuffer, FromBuffer};
use crate::error::{NetCommsError, NetCommsErrorKind};
use crate::ron::{IntoRon, FromRon};

use PacketKind::*;


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

impl IntoBuffer for PacketKind {

    /// This method takes an ownership of self.
    fn into_buff(self) -> Result<Vec<u8>, NetCommsError> {

        let mut buff: Vec<u8> = Vec::new();

        match self {
            Empty => buff.extend([0_u8, 0_u8]),
            MetaData => buff.extend([1_u8, 0_u8]),
            MetaDataEnd => buff.extend([1_u8, 1_u8]),
            Content => buff.extend([2_u8, 0_u8]),
            End => buff.extend([3_u8, 0_u8]),
            Unknown => buff.extend([255_u8, 0_u8]),
        }

        Ok(buff)
    }    
}

impl FromBuffer for PacketKind {

    fn from_buff(buff: Vec<u8>) -> Result<PacketKind, NetCommsError> {

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
