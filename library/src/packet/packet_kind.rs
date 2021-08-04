use serde::{Serialize, Deserialize};

use crate::buffer::{ToBuffer, FromBuffer};
use crate::packet::MetaData;
use crate::packet::PacketKindError;

use PacketKind::*;


/// Each variant determines kind of Packet,
/// some variants also hold data that were or will be transmitted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacketKind {

    Empty,  
    // First data is length of vector, therefore number of bytes in it.
    MetaData(usize, Vec<u8>),   // Here second data hold MetaData struct encoded in RON format.
    Content(usize, Vec<u8>),    // Actual content of second data depends on message kind which is described in MetaData.
    Request,
    End,    // PacketKind::End signalized end of Message. // MOST LIKELY WILL ADD SOME DATA INSIDE IN FUTURE.
    Unknown,
}

impl ToBuffer for PacketKind {

    /// This method takes an ownership of self.
    fn to_buff(self) -> Vec<u8> {

        let mut buff: Vec<u8> = Vec::new();

        // First two bytes describe PacketKind, rest optional content.
        match self {
            Empty => buff.extend([0_u8, 0_u8]),
            MetaData(_, content) => {
                buff.extend([1_u8, 0_u8]);
                buff.extend(content);
            },
            Content(_, content) => {
                buff.extend([2_u8, 0_u8]);
                buff.extend(content);
            },
            Request => buff.extend([3_u8, 0_u8]),
            End => buff.extend([4_u8, 0_u8]),
            Unknown => buff.extend([255_u8, 0_u8]),
        }

        buff
    }    
}

impl FromBuffer for PacketKind {

    fn from_buff(buff: Vec<u8>) -> Self {

        let kind = &buff[0..2];
        let size = buff.len();
        let contents = &buff[2..size];
        
        // First number in kind describes a PackedKind variant,
        // Second number can describe some variance inside a PackedKind variant.
        let kind = match kind[0] {
            0 => PacketKind::Empty,
            1 => PacketKind::MetaData(size, contents.to_vec()),
            2 => PacketKind::Content(size, contents.to_vec()),
            3 => PacketKind::Request,
            4 => PacketKind::End,
            _ => PacketKind::Unknown,            
        };

        kind   
    }      
    
    
}

impl PacketKind {
    
    /// Creates a new PacketKind::MetaData with metadata supplied in argument.
    /// Takes an ownership of metadata.
    pub fn new_metadata(metadata: MetaData) -> Self {

        let content = metadata.to_buff();
        let size = content.len();

        MetaData(size, content)
    }

    /// Creates a new PacketKind::Content with content supplied in argument.
    /// Takes an ownership of content.
    pub fn new_content(content: Vec<u8>) -> Self {
        Content(content.len(), content)
    }
    
    /// Returns a size in number of bytes of contents, not whole packet.
    pub fn get_size(&self) -> usize {

        // Variants without data inside returns as information about kind is not part of contents.
        let size = match self {
            Empty => 0,
            MetaData(size, _) => *size,
            Content(size, _) => *size,
            Request => 0 as usize,
            End => 0 as usize,
            Unknown => 0 as usize,
        };

        size
    }

    /// Returns just kind of PacketKind, data inside are invalid.
    pub fn get_kind(&self) -> PacketKind {

        let kind =  match self {
            Empty => Empty,
            MetaData(..) => MetaData(0, Vec::new()),
            Content(..) => Content(0, Vec::new()),
            Request => Request,
            End => End,
            Unknown => Unknown,
        };

        kind
    }
    
    /// This method takes an ownership of self
    /// and returns content wrapped in Ok() if called on PacketKind::MetaData or PacketKind::Content,
    /// otherwise returns PacketKindError.
    pub fn get_content(self) -> Result<Vec<u8>, PacketKindError> {

        if let MetaData(_, content) | Content(_, content) = self {
            return Ok(content);
        } else {
            return Err(PacketKindError {});
        }
    }


    /// Temporary method to allow Message::receive() work.
    pub fn get_metadata(&self) -> Result<MetaData, PacketKindError> {
        if let MetaData(size, content) = self {
            Ok(MetaData::from_buff(content.to_vec()))            
        } else {
            return Err(PacketKindError {});        
        }
    }

}