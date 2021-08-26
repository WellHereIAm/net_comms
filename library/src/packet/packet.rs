use serde::{Serialize, Deserialize};
use itertools::Itertools;

use std::io::{Read, Write};
use std::net::TcpStream;

use crate::buffer::{IntoBuffer, FromBuffer};
use crate::bytes::{Bytes, IntoBytes};
use crate::error::{NetCommsError, NetCommsErrorKind};
use crate::packet::PacketKind;
use crate::ron::{IntoRon, FromRon};


/// Minimal number of bytes that is every packet guaranteed to have, 2 bytes are for its size and 2 for its kind.
pub const PACKET_DESCRIPTION_SIZE: u16 = 4;

/// Maximum number of bytes that can one packet hold.
/// 
/// Minimum value is 5 bytes, 2 for packet `size`([u16]), 2 for packet [`kind`](PacketKind), and at least 1 for [`content`](Vec). 
/// It is a `mutable` statics so it can be changed for specific needs of user of this framework byt since it is
/// only a [u16] it is capped by [u16::MAX].
/// This also should be declared only once at the start of an application, or even better in content.
pub static mut MAX_PACKET_SIZE: u16 = 1024;

/// Maximum amount of bytes that a [Packet] can use for its content, its lower than [MAX_PACKET_SIZE] by [PACKET_DESCRIPTION_SIZE].
pub static MAX_PACKET_CONTENT_SIZE: u16 = unsafe { MAX_PACKET_SIZE - PACKET_DESCRIPTION_SIZE };

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
    size: u16,    
    kind: PacketKind,  
    content: Bytes,
}

impl IntoRon for Packet {}
impl FromRon<'_> for Packet {}

impl Default for Packet {
    
    fn default() -> Self {
        Packet {
            size: PACKET_DESCRIPTION_SIZE,
            kind: PacketKind::Empty,
            content: Bytes::new(),
        }
    }
}

impl IntoBuffer for Packet {
    
    /// This takes an ownership of self.
    fn into_buff(self) -> Result<Vec<u8>, NetCommsError> {        

        let mut buff: Vec<u8> = Vec::new();
        buff.extend(self.size.into_buff()?);
        buff.extend(self.kind.into_buff()?);
        buff.extend(self.content.into_vec());

        Ok(buff)
    }
}

impl FromBuffer for Packet {

    fn from_buff(buff: Vec<u8>) -> Result<Packet, NetCommsError>{

        // Check if buffer has valid length(at least 4 for kinds without any content).
        if let None = buff.get((PACKET_DESCRIPTION_SIZE - 1) as usize) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation from_buff for Packet requires buffer of length of at least 4 bytes.".to_string())));
        }

        let size = buff.len();
        let kind = PacketKind::from_buff(buff[2..4].to_vec())?; // Starts at 2 because size field takes 2 bytes in buffer.
        let content = buff[4..size].to_vec();

        Ok(Packet {
            size: size as u16,
            kind,
            content: content.into_bytes(),
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
    pub fn new(kind: PacketKind, content: Bytes) -> Self {

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

    pub fn send(self, stream: &mut TcpStream) -> Result<(), NetCommsError> {
        
        let packet_buff = self.into_buff()?;

        if let Err(e) = stream.write(&packet_buff) {
            let packet = Packet::from_buff(packet_buff)?;
            return Err(NetCommsError::new(
                NetCommsErrorKind::WritingToStreamFailed,
                Some(format!("Failed to write packet to stream.\n{}\n{}",
                                        packet.into_ron_pretty(None)?,
                                        e))));
        }

        Ok(())
    }

    fn receive(stream: &mut TcpStream) -> Result<Packet, NetCommsError> {

        // Reads the size of packet.
        let mut size_buff = vec![0_u8; 2];
        if let Err(e) = stream.read_exact(&mut size_buff) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::ReadingFromStreamFailed, 
                Some(format!("Failed to read the size of packet. \n({})", e))));
        }
        let size = u16::from_buff(size_buff.clone())?;

        // Reads rest of packet.
        // - 2 for size of packet encoded as bytes which already exist.
        let mut buff = vec![0_u8; (size - 2) as usize];    
        // USE READ EXACT
        if let Err(e) = stream.read_exact(&mut buff) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::ReadingFromStreamFailed, 
                Some(format!("Failed to read contents of packet. \n({})", e))));
        }

        // Connect whole buffer and change name, so it makes more sense.
        size_buff.extend(buff);
        let buff = size_buff;
        
        // Create and return a packet from buffer.
        Ok(Packet::from_buff(buff)?)
    }

    fn number_of_packets(length: usize) -> u32 {

        // Get number of packets by dividing by MAX_PACKET_CONTENT_SIZE.
        let mut number_of_packets = length / (MAX_PACKET_CONTENT_SIZE as usize);  
        // Add one packet if there is any remainder after the division.
        if length % (MAX_PACKET_CONTENT_SIZE as usize) != 0 {
            number_of_packets += 1;
        }
        number_of_packets as u32
    }

    fn split_to_max_packet_size(buffer: Vec<u8>) -> Vec<Vec<u8>> {

        // This splits given buffer to multiple owned chunks with chunks method from itertools crate,
        // then it will split every chunk to iterator as well which are then collected to vectors of bytes,
        // that are collected to single vector. 
        // This is not my work: https://stackoverflow.com/a/67009164. 
        let vectored_content: Vec<Vec<u8>> = buffer.into_iter()
                                                    .chunks(MAX_PACKET_CONTENT_SIZE as usize)
                                                    .into_iter()
                                                    .map(|chunk| chunk.collect())
                                                    .collect();
     
        vectored_content
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
    pub fn content(&self) -> Bytes {
        self.content.clone()
    }

    /// Consumes `self` and returns `content`.
    pub fn content_move(self) -> Bytes {
        self.content
    }

    /// Maximum amount of bytes that a [Packet] can use for its content, its lower than [MAX_PACKET_SIZE] by [PACKET_DESCRIPTION_SIZE].
    ///
    /// It is an [unsafe] operation since it does access a [mutable](mut) [static]
    pub fn max_content_size() -> u16 {
        unsafe { MAX_PACKET_SIZE - PACKET_DESCRIPTION_SIZE }
    }
}