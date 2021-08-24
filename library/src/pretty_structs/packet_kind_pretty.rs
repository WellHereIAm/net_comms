
use serde::{Serialize, Deserialize};

use crate::ron::{ToRon, FromRon};
use crate::packet::PacketKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacketKindPretty {
    Empty,  
    MetaData,  
    MetaDataEnd, 
    Content, 
    End, 
    Unknown,
}

impl ToRon for PacketKindPretty {}
impl FromRon<'_> for PacketKindPretty {}

impl PacketKindPretty {
    
    pub fn from_packet_kind(packet_kind: &PacketKind) -> Self {
        match packet_kind {
            PacketKind::Empty => Self::Empty,
            PacketKind::MetaData(..) => Self::MetaData,
            PacketKind::MetaDataEnd(..) => Self::MetaDataEnd,
            PacketKind::Content(..) => Self::Content,
            PacketKind::End => Self::Content,
            PacketKind::Unknown => Self::Unknown,
        }
    }
}