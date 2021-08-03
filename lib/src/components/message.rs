use std::{io::{Read, Write}, net::TcpStream};

use serde::{Deserialize, Serialize};

use crate::{FromBuffer, MetaData, Packet, PacketKind, ToBuffer};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub kind: MessageKind,
    pub metadata: MetaData,
    pub content: Vec<Packet>,
    pub end_data: Packet,
}

impl Message {
    
    pub fn new() -> Self {
        Message {
            kind: MessageKind::Unknown,
            metadata: MetaData::new_empty(),
            content: Vec::new(),
            end_data: Packet::new_empty(),
        }
    }

    pub fn send(self, stream: &mut TcpStream) {

        let metadata_packet = Packet::new(PacketKind::new_metadata(self.metadata.clone()));

        stream.write(&metadata_packet.to_buff()).unwrap();
        println!("metadata to buff: {:?}", &self.metadata.to_buff());
        
        for packet in self.content {
            stream.write(&packet.clone().to_buff()).unwrap();
            println!("content to buff: {:?}", &packet.to_buff());  
        }

        stream.write(&self.end_data.clone().to_buff()).unwrap();
        println!("end_data to buff: {:?}", &self.end_data.to_buff()); 
    }

    // Refractor this.
    pub fn receive(stream: &mut TcpStream) -> Self {

        println!("receive()\n");

        let mut msg = Message::new();
        
        loop {

            println!("Loop start\n");
            
            let mut size_buff = vec![0_u8; 8];
            stream.read_exact(&mut size_buff).unwrap();

            // let mut kind_buff = vec![0_u8; 2];
            // stream.read_exact(&mut kind_buff).unwrap();

            println!("Size buffer after read: {:?}", &size_buff);

            let size = usize::from_buff(size_buff.clone());
            let mut buff = vec![0_u8; size - 8];

            stream.read_exact(&mut buff).unwrap();

            println!("Buffer after read: {:?}", &buff);

            size_buff.extend(buff);

            println!("Buffer after whole read: {:?}", &size_buff);

            let packet = Packet::from_buff(size_buff);
            println!("{:?}", &packet);

            // I would like change kind to private later.
            match packet.kind {
                crate::PacketKind::Empty(..) => {},
                crate::PacketKind::MetaData(..) => {
                    msg.set_metadata(packet.kind.get_metadata().unwrap()); // That is just stupid, isn´t it.
                },
                crate::PacketKind::Content(..) => {
                    msg.push_content(packet);
                },
                crate::PacketKind::Request => {
                    println!("Request");
                },
                crate::PacketKind::End => {
                    msg.set_end_data(packet);
                    break;
                },
                crate::PacketKind::Unknown => {
                    println!("Unknown.")
                },
            }  
            println!("{:?}", &msg);
        }         

        msg
    }

    pub fn set_metadata(&mut self, metadata: MetaData) {

        self.kind = metadata.message_kind.clone();
        self.metadata = metadata;
    }

    pub fn push_content(&mut self, content: Packet) {
        self.content.push(content);
    }

    pub fn set_end_data(&mut self, end_data: Packet) {
        self.end_data = end_data;
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


