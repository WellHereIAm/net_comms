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
/// * `size` -- size of the whole [packet](Packet) in number of bytes. It is [u16] so that packet can not have size over [u16::MAX]
/// * `kind` -- [kind](PacketKind) of [packet](Packet). 
/// * `content` -- data stored in the [packet](Packet).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    size: u16,    // Size of whole packet with contents in number of bytes.
    kind: PacketKind,   // Kind of packet, also holds all other packet data.
    content: Vec<u8>,
}

impl ToRon for Packet {}
impl FromRon<'_> for Packet {}

impl ToBuffer for Packet {
    
    /// This takes an ownership of self.
    fn to_buff(self) -> Result<Vec<u8>, NetCommsError> {        

        let mut buff: Vec<u8> = Vec::new();
        buff.extend(self.size.to_buff()?);
        buff.extend(self.kind.to_buff()?);
        buff.extend(self.content);

        Ok(buff)
    }
}

impl FromBuffer for Packet {

    fn from_buff(buff: Vec<u8>) -> Result<Packet, NetCommsError>{

        // Check if buffer has valid length(at least 4 for kinds without any content).
        if let None = buff.get((PACKET_DESCRIPTION_SIZE - 1) as usize) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation from_buff for Packet requires buffer of length of at least 10 bytes.".to_string())));
        }

        let size = buff.len();
        let kind = PacketKind::from_buff(buff[2..4].to_vec())?; // Starts at 2 because size field takes 2 bytes in buffer.
        let content = buff[4..size].to_vec();

        Ok(Packet {
            size: size as u16,
            kind,
            content,
        })
    }
}

impl Packet {

    /// Creates a new [Packet].
    ///
    /// Size of packet is derived from [`kind`](PacketKind) and `content` given.
    ///
    /// # Examples
    /// End packet at the end of the [Message](crate::message::Message) is created like that.
    /// ```
    /// let packet = Packet::new(PacketKind::End);
    /// ```
    pub fn new(kind: PacketKind, content: Vec<u8>) -> Self {

        // Size is composed of three parts:
        // Size of size field which is always 2.
        // Size of PacketKind which is always 2.
        // Size of data inside PacketKind which size is dynamic.
        let size = PACKET_DESCRIPTION_SIZE + content.len() as u16;

        Packet {
            size: size as u16,
            kind,
            content,
        }
    }

    /// Creates a new empty [Packet].
    pub fn new_empty() -> Self {

        Packet {
            size: PACKET_DESCRIPTION_SIZE,
            kind: PacketKind::Empty,
            content: Vec::new(),
        }
    }
    
    /// Returns `size`.
    pub fn size(&self) -> u16 {
        self.size
    }

    /// Returns `kind`.
    pub fn kind(&self) -> PacketKind {
        self.kind.clone()
    }

    /// Returns `content`.
    ///
    /// Content is cloned.
    pub fn content(&self) -> Vec<u8> {
        self.content.clone()
    }

    /// Consumes `self` and returns `content`.
    pub fn content_owned(self) -> Vec<u8> {
        self.content
    }
}