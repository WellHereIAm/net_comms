use serde::{Serialize, Deserialize};

use crate::packet::PacketKind;
use crate::buffer::{ToBuffer, FromBuffer};
use crate::error::{NetCommsError, NetCommsErrorKind};
use crate::config::PACKET_DESCRIPTION_SIZE;


/// Gives structure to data to be sent or received from stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    size: usize,    // Size of whole packet with contents in number of bytes.
    kind: PacketKind,   // Kind of packet, also holds all other packet data.
}

impl ToBuffer for Packet {
    
    /// This takes an ownership of self.
    fn to_buff(self) -> Result<Vec<u8>, NetCommsError> {        

        let mut buff: Vec<u8> = Vec::new();
        buff.extend(self.size.to_buff()?);
        buff.extend(self.kind.to_buff()?);

        Ok(buff)
    }
}

impl FromBuffer for Packet {

    fn from_buff(buff: Vec<u8>) -> Result<Packet, NetCommsError>{

        // Check if buffer has valid length(at least 10).
        match buff.get(PACKET_DESCRIPTION_SIZE - 1) { // - 1 because index starts at 0.
            Some(_) => &buff[0..8],
            None => return Err(NetCommsError {
                kind: NetCommsErrorKind::InvalidBufferLength,
                message: Some("Implementation from_buff for Packet requires buffer of length of at least 11 bytes.".to_string()),
            }),
        };

        let size = buff.len();
        let kind = PacketKind::from_buff(buff[8..size].to_vec())?; // Starts at 8 because size field takes 8 bytes in buffer.

        Ok(Packet {
            size,
            kind,
        })
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
        let size = kind.size() + PACKET_DESCRIPTION_SIZE;

        Packet {
            size,
            kind,
        }
    }

    /// Creates new empty packet.
    pub fn new_empty() -> Self {

        Packet {
            size: PACKET_DESCRIPTION_SIZE,
            kind: PacketKind::Empty
        }
    }
    
    /// Returns size of packet.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns only kind of PacketKind, data inside are invalid.
    /// Wrapper around PacketKind::kind().
    pub fn kind(&self) -> PacketKind {
        self.kind.kind()
    }

    /// This method takes an ownership of self and returns PacketKind with valid data inside.
    pub fn kind_owned(self) -> PacketKind {
        self.kind
    }
}