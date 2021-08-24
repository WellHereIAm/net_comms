use serde::{Serialize, Deserialize};

use crate::buffer::{ToBuffer, FromBuffer};
use crate::error::{NetCommsError, NetCommsErrorKind};
use crate::packet::PacketKind;
use crate::ron::{ToRon, FromRon};
use crate::config::PACKET_DESCRIPTION_SIZE;


/// Gives structure to data to be sent or received from stream.
///
/// [Packet] is the lowest abstraction above buffer in this library.
///
/// # Fields
///
/// * `size` -- size of the whole [packet](Packet) in number of bytes.
/// * `kind` -- kind of [packet](Packet), also hold all other packet data besides its size. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    size: usize,    // Size of whole packet with contents in number of bytes.
    kind: PacketKind,   // Kind of packet, also holds all other packet data.
}

impl ToRon for Packet {}
impl FromRon<'_> for Packet {}

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

        // Check if buffer has valid length(at least 10 for kinds without any content).
        if let None = buff.get(PACKET_DESCRIPTION_SIZE - 1) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation from_buff for Packet requires buffer of length of at least 10 bytes.".to_string())));
        }

        let size = buff.len();
        let kind = PacketKind::from_buff(buff[8..size].to_vec())?; // Starts at 8 because size field takes 8 bytes in buffer.

        Ok(Packet {
            size,
            kind,
        })
    }
}

impl Packet {

    /// Creates a new [Packet].
    ///
    /// Size of packet is derived from [`kind`](PacketKind) given.
    ///
    /// # Examples
    /// End packet at the end of the [Message](crate::message::Message) is created like that.
    /// ```
    /// let packet = Packet::new(PacketKind::End);
    /// ```
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

    /// Creates a new empty [Packet].
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

    /// Returns only `kind` of [PacketKind], data inside are invalid.
    ///
    /// This is a wrapper around [PacketKind::kind], if valid data are needed use [Packet::kind_owned].
    pub fn kind(&self) -> PacketKind {
        self.kind.kind()
    }

    /// This method takes an ownership of self and returns [PacketKind] with valid data inside.
    ///
    /// If valid data inside are not needed and `kind` is only interest, use [Packet::kind].
    pub fn kind_owned(self) -> PacketKind {
        self.kind
    }
}