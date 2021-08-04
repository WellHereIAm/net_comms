use serde::{Serialize, Deserialize};

use crate::packet::PacketKind;
use crate::buffer::{ToBuffer, FromBuffer};


/// Gives structure to data to be sent and received from stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    size: usize,    // Size of whole packet with contents in number of bytes.
    pub kind: PacketKind,   // Kind of packet, also holds all packet data, except the whole packet size.
}

impl ToBuffer for Packet {
    
    /// This takes an ownership of self.
    fn to_buff(self) -> Vec<u8> {

        let mut buff: Vec<u8> = Vec::new();
        buff.extend(self.size.to_buff());
        buff.extend(self.kind.to_buff());

        buff
    }
}

impl FromBuffer for Packet {

    fn from_buff(buff: Vec<u8>) -> Self {

        let size = buff.len();
        let kind = PacketKind::from_buff(buff[8..size].to_vec()); // Starts at 8 because size field takes 8 bytes in buffer.

        Packet {
            size,
            kind,
        }
    }
}

impl Packet {

    /// Creates new Packet with given PacketKind.
    /// Size of packet is derived from PacketKind given.
    pub fn new(kind: PacketKind) -> Self {

        // Size is composed of three parts:
        // Size of size field which is always 8.
        // Size of PacketKind which is always 2.
        // Size of data inside PacketKind which size is dynamic.
        let size = kind.size() + 10;

        Packet {
            size,
            kind,
        }
    }

    /// Creates new empty packet.
    pub fn new_empty() -> Self {

        Packet {
            size: 10,
            kind: PacketKind::Empty // THIS SHOULD NOT HAVE ANY DATA INSIDE - CHANGE!
        }
    }
    
    /// Returns size of packet.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns just kind of PacketKind, data inside are invalid.
    /// Wrapper around PacketKind::get_kind().
    pub fn kind(&self) -> PacketKind {
        self.kind.kind()
    }

    /// This method takes an ownership of self
    /// and returns PacketKind with valid data inside.
    pub fn kind_owned(self) -> PacketKind {
        self.kind
    }
}