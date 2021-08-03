use std::{io::{Read, Write}, net::TcpStream};

use serde::{Deserialize, Serialize};

use crate::{FromBuffer, Packet, PacketKind, ToBuffer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageKind {
    Empty,
    Request,
    Text,
    File,
    Unknown,
}

impl ToBuffer for MessageKind {

    fn to_buff(self) -> Vec<u8> {

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
    pub kind: MessageKind,
    pub metadata: Packet,
    pub add_info: Vec<Packet>,
    pub content: Vec<Packet>,
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

        stream.write(&self.metadata.clone().to_buff()).unwrap();
        println!("metadata to buff: {:?}", &self.metadata.to_buff());

        for packet in self.add_info {
            stream.write(&packet.clone().to_buff()).unwrap();
            println!("add_info to buff: {:?}", &packet.to_buff());
        }
        
        for packet in self.content {
            stream.write(&packet.clone().to_buff()).unwrap();
            println!("content to buff: {:?}", &packet.to_buff());  
        }
    }

    // Refractor this.
    pub fn receive(stream: &mut TcpStream) -> Self {

        let mut msg = Message::new();
        
        let mut packet_id = 0;

        loop {
            let mut size_buff = vec![0_u8; 8];
            stream.read_exact(&mut size_buff).unwrap();
            let size = usize::from_buff(size_buff.clone());
            let mut buff = vec![0_u8; size - 8];

            packet_id += 1;

            stream.read_exact(&mut buff).unwrap();
            size_buff.extend(buff);

            let packet = Packet::from_buff(size_buff);

            match packet.kind {
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
                    msg.push_content(packet);
                },
                crate::PacketKind::Unknown => {
                    println!("Unknown.")
                },
            }  
            
            if let PacketKind::MetaData(length, ..) = msg.metadata.kind {
                if packet_id == length {
                    break;
                }
            }
        }         

        msg
    }

    pub fn set_kind(&mut self, kind: MessageKind) {
        self.kind = kind
    }

    pub fn set_metadata(&mut self, metadata: Packet) {
        if let PacketKind::MetaData(_, kind, ..) = metadata.kind.clone() {
            self.kind = kind;
        }
        self.metadata = metadata
    }

    pub fn push_add_info(&mut self, add_info: Packet) {
        self.add_info.push(add_info)
    }

    pub fn push_content(&mut self, content: Packet) {
        self.content.push(content)
    }

    pub fn get_content(self) ->  Vec<u8> {
        let mut content: Vec<u8> = Vec::new();
        for data in self.content.into_iter() {
            if let PacketKind::Content(_, data) = data.get_contents() {
                content.extend(data);
            }
        }
        content
    }
}


