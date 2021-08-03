// Finish errors, now they are not very helpful. Maybe use one error struct for whole project and just use codes?


use std::{time::SystemTime};

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::{FromBuffer, MessageKind, ToBuffer};

use PacketKind::*;

type Size = usize;
type Id = usize;
type Length = usize;
type Content = Vec<u8>;

// Change whole concept of metadata and additional info to one metadata json? Probably better choice after I make it work.
#[derive(Debug, Clone)]
pub enum PacketKind {
    Empty(Size, Content),
    MetaData(Length, MessageKind, Id, Id, DateTime<Utc>),
    AddInfo(Size, Content),
    Content(Size, Content),
    Request,
    End,
    Unknown,
}

impl ToBuffer for PacketKind {

    fn to_buff(self) -> Vec<u8> {

        let mut buff: Vec<u8> = Vec::new();

        match self {
            Empty(_, content) => {
                buff.extend([0_u8, 0_u8]);
                buff.extend(content);
            },
            MetaData(length, msg_kind, author_id, recipient_id, datetime) => {
                buff.extend([1_u8, 0_u8]);
                buff.extend(length.to_buff());
                buff.extend(msg_kind.to_buff());
                buff.extend(author_id.to_buff());
                buff.extend(recipient_id.to_buff());
                buff.extend(datetime.to_buff());
            },
            AddInfo(_, content) => {
                buff.extend([2_u8, 0_u8]);
                buff.extend(content);
            },
            Content(_, content) => {
                buff.extend([3_u8, 0_u8]);
                buff.extend(content);
            },
            Request => {
                buff.extend([4_u8, 0_u8]);
            },
            End => {
                buff.extend([5_u8, 0_u8]);
            },
            Unknown => {
                buff.extend([255_u8, 0_u8]);
            },
        }

        buff
    }    
}

impl FromBuffer for PacketKind {

    fn from_buff(buff: Vec<u8>) -> Self {

        let kind = &buff[0..2];
        let size = buff.len();
        let contents = &buff[2..size];
        
        let kind = match kind[0] {
            0 => PacketKind::Empty(size, contents.to_vec()),
            1 => PacketKind::MetaData(
                usize::from_buff(contents[0..8].to_vec()),
                MessageKind::from_buff(contents[8..10].to_vec()),
                usize::from_buff(contents[10..18].to_vec()),
                usize::from_buff(contents[18..26].to_vec()),
                DateTime::<Utc>::from_buff(contents[26..34].to_vec()),
            ),
            2 => PacketKind::AddInfo(size, contents.to_vec()),
            3 => PacketKind::Content(size, contents.to_vec()),
            4 => PacketKind::Request,
            5 => PacketKind::End,
            _ => PacketKind::Unknown,            
        };

        kind   
    }      
    
    
}

// General implementation
impl PacketKind {

    pub fn new_empty(size: usize) -> Self {
        Empty(size, vec![0_u8; size])
    }

    pub fn new_metadata(length: usize, msg_kind: MessageKind, author_id: usize, recipient_id: usize) -> Self {
        MetaData(length, msg_kind, author_id, recipient_id, PacketKind::get_datetime())
    }

    pub fn new_add_info(content: Vec<u8>) -> Self {
        AddInfo(content.len(), content)
    }

    pub fn new_content(content: Vec<u8>) -> Self {
        Content(content.len(), content)
    }

    pub fn get_size(&self) -> usize {

        let size = match self {
            Empty(size, _) => *size,
            MetaData(..) => 34 as usize,
            AddInfo(size, _) => *size,
            Content(size, _) => *size,
            Request => 0 as usize,
            End => 0 as usize,
            Unknown => 0 as usize,
        };

        size
    }

    /// This returns just kind, data inside are invalid.
    pub fn get_kind(&self) -> PacketKind {

        let kind =  match self {
            Empty(..) => Empty(0, Vec::new()),
            MetaData(..) => PacketKind::new_metadata(0, MessageKind::Empty, 0, 0),
            AddInfo(..) => AddInfo(0, Vec::new()),
            Content(..) => Content(0, Vec::new()),
            Request => Request,
            End => End,
            Unknown => Unknown,
        };

        kind
    }

    fn get_datetime() -> DateTime<Utc> {
    
        let now = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();

        let naive_datetime = NaiveDateTime::from_timestamp(now as i64, 0);
    
        let time = DateTime::from_utc(naive_datetime, Utc); 
    
        time
    }
    
}

// Implementation for Empty, AddInfo, Content.
impl PacketKind {

    /// !!!
    /// This method gets an ownership of self.
    pub fn get_content(self) -> Result<Vec<u8>, PacketKindError> {

        if let Empty(_, content) | AddInfo(_, content) | Content(_, content) = self {
            return Ok(content);
        } else {
            return Err(PacketKindError {});
        }
    }
}

// Implementation for MetaData variant.
impl PacketKind {

    pub fn get_message_length(&self) -> Result<usize, PacketKindError> {

        if let MetaData(length, ..) = self {
            return Ok(*length);
        } else {
            return Err(PacketKindError {});
        }
    }

    pub fn get_message_kind(&self) -> Result<MessageKind, PacketKindError> {

        if let MetaData(_, msg_kind, _, _, _) = self {
            return Ok(msg_kind.clone());
        } else {
            return Err(PacketKindError {});
        }
    }

    pub fn get_author_id(&self) -> Result<usize, PacketKindError> {

        if let MetaData(_, _, author_id, _, _) = self {
            return Ok(*author_id);
        } else {
            return Err(PacketKindError {});
        }
    }

    pub fn get_recipient_id(&self) -> Result<usize, PacketKindError> {

        if let MetaData(_, _, _, recipient_id, _) = self {
            return Ok(*recipient_id);
        } else {
            return Err(PacketKindError {});
        }
    }
    
    pub fn get_time(&self) -> Result<DateTime<Utc>, PacketKindError> {

        if let MetaData(.., datetime) = self {
            return Ok(*datetime);
        } else {
            return Err(PacketKindError {});
        }
    }
    
}

#[derive(Debug)]
pub struct PacketKindError {}

impl std::fmt::Display for PacketKindError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PacketKindError")
    }    
}

impl std::error::Error for PacketKindError {}

#[derive(Debug, Clone)]
pub struct Packet {
    size: usize,
    pub kind: PacketKind,
}

impl ToBuffer for Packet {
    
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
        let kind = PacketKind::from_buff(buff[8..size].to_vec());

        Packet {
            size,
            kind,
        }
    }
}

impl Packet {

    pub fn new(kind: PacketKind) -> Self {

        // Byte size of packed 8 for storing it´s size 2 for kind and get_size gets size of data inside each variant.
        let size = kind.get_size() + 10;

        Packet {
            size,
            kind,
        }
    }

    pub fn new_empty() -> Self {

        Packet {
            size: 10,
            kind: Empty(0, Vec::new())
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    /// !!!
    /// This method gets an ownership of self.
    pub fn get_contents(self) -> PacketKind {
        self.kind
    }
}

#[derive(Debug)]
pub struct PacketError {}

impl std::fmt::Display for PacketError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PacketError")
    }    
}

impl std::error::Error for PacketError {}
