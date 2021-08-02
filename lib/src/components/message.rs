use std::{io::{Read, Write}, net::TcpStream};

use crate::{ADDR, FromBuffer, PORT, Packet, ToBuffer};

#[derive(Debug, Clone)]
pub enum MessageKind {
    Empty,
    Request,
    Text,
    File,
    Unknown,
}

impl ToBuffer for MessageKind {

    fn to_buff(&self) -> Vec<u8> {

        let msg_kind = match self {
            MessageKind::Empty => [0_u8, 0_u8],
            MessageKind::Request => [1_u8, 0_u8],
            MessageKind::Text => [2_u8, 0_u8],
            MessageKind::File => [3_u8, 0_u8],
            MessageKind::Unknown => [255_u8, 0_u8],
        };
        msg_kind.to_vec()
    }    
}

impl FromBuffer for MessageKind {
    
    fn from_buff(buff: Vec<u8>) -> Self {

        let msg_kind = match buff[0] {
            0 => MessageKind::Empty,
            1 => MessageKind::Request,
            2 => MessageKind::Text,
            3 => MessageKind::File,
            _ => MessageKind::Unknown,            
        }; 
        
        msg_kind
    }
}

#[derive(Debug)]
pub struct Message {
    kind: MessageKind,
    metadata: Packet,
    add_info: Vec<Packet>,
    content: Vec<Packet>,
}

impl Message {
    
    pub fn new() -> Self {
        Message {
            kind: MessageKind::Unknown,
            metadata: Packet::new_empty(),
            add_info: Vec::new(),
            content: Vec::new(),
        }
    }

    pub fn send(self, stream: &mut TcpStream) {

        stream.write(&self.metadata.to_buff()).unwrap();

        for packet in self.add_info {
            stream.write(&packet.to_buff()).unwrap();
        }
        
        for packet in self.content {
            stream.write(&packet.to_buff()).unwrap();  
        }
    }

    pub fn receive(stream: &mut TcpStream) -> Self {

        let mut msg = Message::new();

        loop {
            let mut buff = vec![0_u8; 8];
            stream.read_exact(&mut buff).unwrap();
            let size = usize::from_buff(buff);

            let mut buff = vec![0_u8; size];
            stream.read_exact(&mut buff).unwrap();

            let packet = Packet::from_buff(buff);

            match packet.get_contents() {
                crate::PacketKind::Empty(..) => {},
                crate::PacketKind::MetaData(..) => {
                    msg.set_metadata(packet);
                },
                crate::PacketKind::AddInfo(..) => {
                    msg.push_add_info(packet);
                },
                crate::PacketKind::Content(..) => {
                    msg.push_content(packet);
                },
                crate::PacketKind::Request => {
                    println!("Request");
                },
                crate::PacketKind::End => {
                    break;
                },
                crate::PacketKind::Unknown => {
                    println!("Unknown.")
                },
            }
        }         

        msg
    }

    pub fn set_kind(&mut self, kind: MessageKind) {
        self.kind = kind
    }

    pub fn set_metadata(&mut self, metadata: Packet) {
        self.metadata = metadata
    }

    pub fn push_add_info(&mut self, add_info: Packet) {
        self.add_info.push(add_info)
    }

    pub fn push_content(&mut self, content: Packet) {
        self.content.push(content)
    }
}


