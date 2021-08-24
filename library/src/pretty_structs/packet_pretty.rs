
use serde::{Serialize, Deserialize};

use crate::buffer::FromBuffer;
use crate::packet::{Packet, PacketKind};
use crate::ron::{ToRon, FromRon};
use crate::pretty_structs::PacketKindPretty;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketPretty{
    size: usize,
    kind: PacketKindPretty,
    content: String,
}

impl ToRon for PacketPretty {}
impl FromRon<'_> for PacketPretty {}

impl PacketPretty {
    
    pub fn from_packet(packet: &Packet) -> Self {

        let content = match packet.clone().kind_owned() {
            PacketKind::Empty => String::new(),
            PacketKind::MetaData(_, content) => String::from_buff(content).unwrap(),
            PacketKind::MetaDataEnd(_, content) => String::from_buff(content).unwrap(),
            PacketKind::Content(_, content) => String::from_buff(content).unwrap(),
            PacketKind::End => String::new(),
            PacketKind::Unknown => String::new(),
        };

        PacketPretty {
            size: packet.size(),
            kind: PacketKindPretty::from_packet_kind(&packet.kind()),
            content,
        }
    }
}

