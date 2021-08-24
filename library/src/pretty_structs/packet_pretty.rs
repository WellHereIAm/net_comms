
use serde::{Serialize, Deserialize};

use crate::buffer::FromBuffer;
use crate::packet::{Packet, PacketKind};
use crate::ron::{ToRon, FromRon};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketPretty{
    size: u16,
    kind: PacketKind,
    content: String,
}

impl ToRon for PacketPretty {}
impl FromRon<'_> for PacketPretty {}

impl PacketPretty {
    
    pub fn from_packet(packet: &Packet) -> Self {

        PacketPretty {
            size: packet.size(),
            kind: packet.kind(),
            content: String::from_buff(packet.content()).unwrap(),
        }
    }
}

