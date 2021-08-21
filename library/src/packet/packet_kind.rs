use serde::{Serialize, Deserialize};

use crate::buffer::{ToBuffer, FromBuffer};
use crate::error::{NetCommsError, NetCommsErrorKind};

use PacketKind::*;


/// Each variant determines kind of [Packet](super::Packet).
///
/// Some variants also hold data to be sent or received from stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacketKind {

    /// Empty packet usually used only for testing or like a placeholder.
    Empty,  

    /// First [usize] is length of [Vec] of [u8], which holds content of this [Packet](super::Packet),
    /// in this case part of [MetaData](super::MetaData) encoded in [RON](ron) format.
    MetaData(usize, Vec<u8>),  

    /// First [usize] is length of [Vec] of [u8], which holds content of this [Packet](super::Packet),
    /// in this case last part of [MetaData](super::MetaData) encoded in [RON](ron) format.
    /// It is same as [MetaData], differing only in fact that this variant is used to sign the last part of [MetaData](super::MetaData).
    MetaDataEnd(usize, Vec<u8>), 

    /// First [usize] is length of [Vec] of [u8], which holds content of this [Packet](super::Packet),
    /// actual content depends on [MessageKind](crate::message::MessageKind) which is described in metadata of related
    /// [Message](crate::message::Message)
    Content(usize, Vec<u8>), 

    /// Used to signalize end of the [Message](crate::message::Message)
    End, 

    /// Used in case if [Packet](super::Packet) fails to recognize a [PacketKind].
    Unknown,
}

impl ToBuffer for PacketKind {

    /// This method takes an ownership of self.
    fn to_buff(self) -> Result<Vec<u8>, NetCommsError> {

        let mut buff: Vec<u8> = Vec::new();

        // First two bytes describe PacketKind, rest is an optional content.
        match self {
            Empty => buff.extend([0_u8, 0_u8]),
            MetaData(_, content) => {
                buff.extend([1_u8, 0_u8]);
                buff.extend(content);
            },
            MetaDataEnd(_, content) => {
                buff.extend([1_u8, 1_u8]);
                buff.extend(content);
            }
            Content(_, content) => {
                buff.extend([2_u8, 0_u8]);
                buff.extend(content);
            },
            End => buff.extend([3_u8, 0_u8]),
            Unknown => buff.extend([255_u8, 0_u8]),
        }

        Ok(buff)
    }    
}

impl FromBuffer for PacketKind {

    fn from_buff(buff: Vec<u8>) -> Result<PacketKind, NetCommsError> {

        // Check if buffer has valid length(at least 2).
        let kind = match buff.get(1) {
            Some(_) => &buff[0..2],
            None => return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidBufferSize,
                Some("Implementation from_buff for PacketKind requires buffer of length of at least three bytes.".to_string())))
        };

        // Here is necessary to get whole buffer size,
        // but when size is written to PacketKind we need to remove 2 for kind.
        let buffer_size = buff.len();  
        let content_size = buffer_size - 2;    // Minus two bytes which describe kind.
        let contents = &buff[2..buffer_size];
        
        // First number in kind describes a PackedKind variant,
        // Second number can describe some variance inside a PackedKind variant.
        let kind = match kind[0] {
            0 => PacketKind::Empty,
            1 => match kind[1] {
                    0 => PacketKind::MetaData(content_size, contents.to_vec()),
                    1 => PacketKind::MetaDataEnd(content_size, contents.to_vec()),
                    _ => PacketKind::Unknown,              
            }
            2 => PacketKind::Content(content_size, contents.to_vec()),
            3 => PacketKind::End,
            _ => PacketKind::Unknown,            
        };

        Ok(kind)   
    }      
    
    
}

impl PacketKind {
    
    /// Creates a new [PacketKind::MetaData].
    ///
    /// Takes an ownership of content, which is [RON](ron) encoded [MetaData](super::MetaData) converted to buffer.
    pub fn new_metadata(content: Vec<u8>) -> Self {

        let size = content.len();

        MetaData(size, content)
    }

    /// Creates a new [PacketKind::MetaDataEnd].
    ///
    /// Takes an ownership of content, which is [RON](ron) encoded [MetaData](super::MetaData) converted to buffer.
    pub fn new_metadata_end(content: Vec<u8>) -> Self {

        let size = content.len();

        MetaDataEnd(size, content)
    }


    /// Creates a new [PacketKind::Content].
    /// 
    /// Takes an ownership of content.
    pub fn new_content(content: Vec<u8>) -> Self {
        Content(content.len(), content)
    }
    
    /// Return size of `content` in number of bytes.
    ///
    /// Variants without any `content` return 0. 
    ///
    /// # Examples
    /// This is an example for variant with a `content`.
    /// ```
    /// # use library::buffer::ToBuffer;
    /// # use library::packet::{Packet, PacketKind};
    /// let content = "Hello.".to_string().to_buff().unwrap();
    /// let packet_kind = PacketKind::new_content(content);
    /// assert_eq!(6, packet_kind.size());
    /// ```
    /// This is an example for variant without a `content`.
    /// ```
    /// # use library::buffer::ToBuffer;
    /// # use library::packet::{Packet, PacketKind};
    /// let packet_kind = PacketKind::End;
    /// assert_eq!(0, packet_kind.size());
    /// ```
    pub fn size(&self) -> usize {
        
        let size = match self {
            Empty => 0,
            MetaData(size, _) => *size,
            MetaDataEnd(size, _) => *size,
            Content(size, _) => *size,
            End => 0 as usize,
            Unknown => 0 as usize,
        };

        size
    }

    /// Returns only `kind` of [PacketKind], data inside are invalid.
    ///
    /// To get valid data use [PacketKind::kind_owned].
    pub fn kind(&self) -> PacketKind {

        let kind =  match self {
            Empty => Empty,
            MetaData(..) => MetaData(0, Vec::new()),
            MetaDataEnd(..) => MetaDataEnd(0, Vec::new()),
            Content(..) => Content(0, Vec::new()),
            End => End,
            Unknown => Unknown,
        };

        kind
    }

    /// Returns `kind` of [PacketKind], with valid data inside.
    ///
    /// This takes an ownership of self.
    /// If valid data inside are not needed and `kind` is only interest, use [PacketKind::kind].
    pub fn kind_owned(self) -> PacketKind {
        let kind =  match self {
            Empty => Empty,
            MetaData(size, content) => MetaData(size, content),
            MetaDataEnd(size, content) => MetaDataEnd(size, content),
            Content(size, content) => Content(size, content),
            End => End,
            Unknown => Unknown,
        };

        kind
    }

    /// Returns `content`.
    ///
    /// This can be used only on variants with `content`.
    ///
    /// # Errors
    ///
    /// * If called on [PacketKind] without `content` [NetCommsError] with kind [NetCommsErrorKind::InvalidPacketKind] is returned.
    pub fn content(self) -> Result<Vec<u8>, NetCommsError> {

        if let MetaData(_, content) | MetaDataEnd(_, content) | Content(_, content) = self {
            return Ok(content);
        } else {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidPacketKind,
                Some("This can be used only on variants MetaData, MetaDataEnd, Content.".to_string())));
        }
    }
}