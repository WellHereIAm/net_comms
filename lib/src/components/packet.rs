use std::{io::Read, net::TcpStream, ops::Range, time::SystemTime};

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::{ADDR, FromBuffer, PORT, ToBuffer, buffer, settings};


#[derive(Debug, Clone)]
pub enum PacketKind {
    Empty,
    Metadata,
    AddInfo,
    Content,
    Request,
    Unknown,
}

impl ToBuffer for PacketKind {

    fn to_buff(&self) -> Vec<u8> {

        let mut buff =  vec![0_u8, 0_u8];


        match self {
            PacketKind::Empty => {},
            PacketKind::Metadata => {
                buff[0] = 1_u8;
                buff[1] = 0_u8;
            },
            PacketKind::AddInfo => {
                buff[0] = 2_u8;
                buff[1] = 0_u8;
            },
            PacketKind::Content => {
                buff[0] = 3_u8;
                buff[1] = 0_u8;
            },
            PacketKind::Request => {
                buff[0] = 4_u8;
                buff[1] = 0_u8;
            },
            PacketKind::Unknown => {
                buff[0] = 255_u8;
                buff[1] = 0_u8;
            }
        }

        buff
    }    
}

impl FromBuffer for PacketKind {

    fn from_buff(buff: Vec<u8>) -> Self {
        
        let kind = match buff[0] {
            0 => PacketKind::Empty,
            1 => PacketKind::Metadata,
            2 => PacketKind::AddInfo,
            3 => PacketKind::Content,
            4 => PacketKind::Request,
            _ => PacketKind::Unknown,            
        };

        kind   
    }      
    
    
}

pub trait PacketType {

    fn get_kind(&self) -> PacketKind;

    fn get_size(&self) -> usize;
}

#[derive(Debug)]
pub struct PacketRaw {
    size: usize,
    kind: PacketKind,
    data: Vec<u8>,
}

impl PacketRaw {
    
    pub fn new() -> Self {
        todo!()
    }

    // later return result
    pub fn get() -> PacketRaw {

        let mut size = 10;
        let mut kind = PacketKind::Unknown;
        let mut data = vec![0_u8; 10];

        

        PacketRaw {
            size,
            kind,
            data
        }
    }
}

#[derive(Debug)]
pub struct Packet<T: PacketType> {
    size: usize,
    kind: PacketKind,
    data: T,
}

impl<T: PacketType + ToBuffer> ToBuffer for Packet<T> {

    fn to_buff(&self) -> Vec<u8> {

        let mut buff: Vec<u8> = Vec::new();

        buffer::write_to_buff(&mut buff, self.size.to_buff().as_slice());
        buffer::write_to_buff(&mut buff, self.kind.to_buff().as_slice());

        buff.extend(self.data.to_buff());

        buff
    }
}

impl <T: PacketType> FromBuffer for Packet<T> {

    fn from_buff(buff: Vec<u8>) -> Self {
        todo!()
    }
}

impl<T: PacketType> Packet<T> {

    fn new_empty() -> Self {
        todo!()
    }

    pub fn new(packet: T) -> Self {
        
        let size = packet.get_size();
        let kind = packet.get_kind();
        let data = packet;

        Packet {
            size,
            kind,
            data
        }
    }

    pub fn get() -> Self {

        todo!()
    }

    pub fn get_size(&self) -> usize {     
        self.size
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn get_kind(&self) -> PacketKind {
        self.kind.clone()  
    }
    
    pub fn set_kind(&mut self, kind: PacketKind) {
        self.kind = kind;
    }

    /// This function needs to be the last one, as it consumes Packet.
    pub fn get_data(self) -> T {
        self.data
    }

    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }
}

impl Packet<EmptyPacket> {

    fn get_empty() -> Packet<EmptyPacket> {
        todo!()
    }    
}

#[derive(Debug, Clone)]
pub struct EmptyPacket {
    size: usize,
    content: Vec<u8>,
}

impl PacketType for EmptyPacket {

    fn get_kind(&self) -> PacketKind {
        PacketKind::Empty
    }

    fn get_size(&self) -> usize {
        self.size
    }
}

impl ToBuffer for EmptyPacket {

    fn to_buff(&self) -> Vec<u8> {
 
        let mut buff: Vec<u8> = Vec::new();

        buffer::write_to_buff(&mut buff, &self.content.as_slice());        

        buff
    }
}

impl FromBuffer for EmptyPacket {

    fn from_buff(buff: Vec<u8>) -> Self {

        // let kind = PacketKind::Empty;
        
        let size = buff.len();
        let content = buff;

        EmptyPacket {
            size,
            content,
        }   
    }
}

impl EmptyPacket {

    pub fn new(size: usize) -> Self {

        let content = vec![0_u8; size];

        EmptyPacket {
            size,
            content,
        }        
    }
    
    pub fn get_content(&self) -> Vec<u8> {
        self.content.clone()
    }
}

#[derive(Debug)]
pub struct MetaDataPacket {
    size: usize, 
    author_id: usize, // Byte size: 8
    recipient_id: usize, // Byte size: 8 
    time: DateTime<Utc>, // Byte size:  8
}

impl PacketType for MetaDataPacket {
    
    fn get_kind(&self) -> PacketKind {
        PacketKind::Metadata
    }

    fn get_size(&self) -> usize {
        self.size
    }

    
}

impl ToBuffer for MetaDataPacket {

    fn to_buff(&self) -> Vec<u8> {

        let mut buff: Vec<u8> = Vec::new();

        buffer::write_to_buff(&mut buff, &self.author_id.to_buff().as_slice());
        buffer::write_to_buff(&mut buff, &self.recipient_id.to_buff().as_slice());
        buffer::write_to_buff(&mut buff, &self.time.to_buff().as_slice());

        buff
    }
    
}
impl FromBuffer for MetaDataPacket {

    fn from_buff(buff: Vec<u8>) -> Self {

        let size = buff.len();
        let author_id = usize::from_buff(buff[0..8].to_vec());
        let recipient_id = usize::from_buff(buff[8..16].to_vec());
        let time = DateTime::<Utc>::from_buff(buff[16..24].to_vec());
        
        MetaDataPacket {
            size,
            author_id,
            recipient_id,
            time,
        }
    }   
}

impl MetaDataPacket {

    pub fn new(author_id: usize) -> Self {

        let size = 24;
        let recipient_id = settings::SERVER_ID;
        let time = Self::get_datetime();

        MetaDataPacket {
            size,
            author_id,
            recipient_id,
            time,
        }
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

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_author_id(&self) -> usize {
        self.author_id
    }

    pub fn get_recipient_id(&self) -> usize {
        self.recipient_id
    }

    pub fn get_time(&self) -> DateTime<Utc> {
        self.time
    }
    
}
pub struct AddInfoPacket;

#[derive(Debug)]
pub struct ContentPacket {
    size: usize,
    content: Vec<u8>,
}

impl PacketType for ContentPacket {

    fn get_kind(&self) -> PacketKind {
        PacketKind::Content
    }

    fn get_size(&self) -> usize {
        self.size
    }
}

impl ToBuffer for ContentPacket {

    fn to_buff(&self) -> Vec<u8> {
 
        let mut buff: Vec<u8> = Vec::new();

        buffer::write_to_buff(&mut buff, &self.content.as_slice());        

        buff
    }
}

impl FromBuffer for ContentPacket {

    fn from_buff(buff: Vec<u8>) -> Self {

        // let kind = PacketKind::Empty;
        
        let size = buff.len();
        let content = buff;

        ContentPacket {
            size,
            content,
        }   
    }
}

impl ContentPacket {

    fn new(size: usize) -> Self {

        let content = vec![0_u8; size];

        ContentPacket {
            size,
            content,
        }        
    }
    
    pub fn get_content(&self) -> Vec<u8> {
        self.content.clone()
    }
}
pub struct RequestPacket;

#[derive(Debug)]
pub struct UnknownPacket;

impl PacketType for UnknownPacket {

    fn get_kind(&self) -> PacketKind {
        PacketKind::Unknown
    }

    fn get_size(&self) -> usize {
        0
    }

    
}

impl UnknownPacket {

    pub fn new() -> Self {
        UnknownPacket {}
    }    
}

