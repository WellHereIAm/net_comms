use chrono::{DateTime, NaiveDateTime, Utc};
use crate::settings::*;

pub enum PacketKind {
    Empty,
    Metadata,
    AddInfo,
    Content,
    Request,
    Unknown,
}

pub enum FileKind {
    Empty,
    Txt,
    Exe, 
    Unknown,    
}

impl FileKind {
    fn to_buff(&self) -> [u8; 2] {
        match self {
            &FileKind::Empty => todo!(),
            FileKind::Txt => todo!(),
            FileKind::Exe => todo!(),
            FileKind::Unknown => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Packet {
    kind: [u8; 2],
    time: [u8; 8],
    packet_id: [u8; 40],
    author_id: [u8; 4],
    recipient_id: [u8; 4],
    other: [u8; 6],
    content: [u8; 960],
}

impl Default for Packet {
    fn default() -> Self {
        todo!()
    }
}

impl Packet {
    pub fn new(kind: PacketKind, author_id: [u8; 4], recipient_id: [u8; 4], content: [u8; 950]) -> Self {
        todo!();
    }

    pub fn new_empty() -> Self {
        Packet {
            kind: [0; 2],
            time: [0; 8],
            packet_id: [0; 40],
            author_id: [0; 4],
            recipient_id: [0; 4],
            other: [0; 6],
            content: [0; 960],
        }
    }

    pub fn from_buff(buff: [u8; BUFFER_SIZE]) -> Self {

        use std::convert::TryInto;

        let kind: [u8; 2] = buff[0..2].try_into().expect("kind");
        let time: [u8; 8] = buff[1..9].try_into().expect("time");
        let packet_id: [u8; 40] = buff[13..63].try_into().expect("packet_id");
        let author_id: [u8; 4] = buff[9..13].try_into().expect("author_id");
        let recipient_id: [u8; 4] = buff[64..68].try_into().expect("recipient_id");
        let other: [u8; 6] = buff[68..74].try_into().expect("other");
        let content: [u8; 960] = buff[74..1024].try_into().expect("content");

        Packet {
            kind,
            time,
            packet_id,
            author_id,
            recipient_id,
            other,
            content,
        }
    }

    fn write_to_buff(buff: &mut [u8; BUFFER_SIZE], arr: &[u8], mut index: usize) -> usize {

        for byte in arr.iter() {
            buff[index] = *byte;
            index += 1;
        }
        index
    }
    
    pub fn to_buff(&self) -> [u8; BUFFER_SIZE] {
        
        let mut buff: [u8; BUFFER_SIZE] = [0_u8; BUFFER_SIZE];
        let mut index = 0;

        index = Packet::write_to_buff(&mut buff, &self.kind, index);
        index = Packet::write_to_buff(&mut buff, &self.time, index);
        index = Packet::write_to_buff(&mut buff, &self.packet_id, index);
        index = Packet::write_to_buff(&mut buff, &self.author_id, index);
        index = Packet::write_to_buff(&mut buff, &self.recipient_id, index);
        index = Packet::write_to_buff(&mut buff, &self.other, index);
        Packet::write_to_buff(&mut buff, &self.content, index);
        
        buff
    }

    pub fn get_kind(&self) -> PacketKind {

        let kind = match self.kind[0] {
            0 => PacketKind::Empty,
            1 => PacketKind::Metadata,
            2 => PacketKind::AddInfo,
            3 => PacketKind::Request,
            4 => PacketKind::Content,
            _ => PacketKind::Unknown,
        };

        kind
    }

    pub fn get_time(&self) -> DateTime<Utc> {

        let naive_datetime = NaiveDateTime::from_timestamp(i64::from_ne_bytes(self.time), 0);

        DateTime::from_utc(naive_datetime, Utc)  
    }

    pub fn get_packet_id(&self) -> u64 {

        let mut id = String::new();
        for x in self.packet_id.iter() {
            id.push_str(format!("{}", x).as_str());            
        }

        id.parse::<u64>().unwrap()
    }

    pub fn get_author_id(&self) -> u64 {

        let mut id = String::new();
        for x in self.author_id.iter() {
            id.push_str(format!("{}", x).as_str());            
        }

        id.parse::<u64>().unwrap()
    }

    fn get_author_username(&self) -> String {
        // Ask server for name from id? Should not the message carry it with itself? or even better should client even know author id?
        todo!()
    }

    fn get_recipient_id() -> u64 {
        todo!()
    }

    fn get_content() -> [u8; 960] {
        todo!()
    }

    fn set_kind() {}
    fn set_time() {}
    fn set_socket_id() {}
    fn set_author_id() {}
    fn set_recipient_id() {}
    fn set_content() {}
}

pub fn get_current_time() -> [u8; 8] {

    use std::time::SystemTime;

    let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
    now.to_ne_bytes()       
}
