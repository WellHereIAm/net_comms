use std::io::{Read, Write};
use std::net::TcpStream;

use serde::{Serialize, Deserialize};

use crate::buffer::{ToBuffer, FromBuffer};
use crate::message::MessageKind;
use crate::packet::{MetaData, PacketKind, Packet};


/// Struct holds all information about message to be sent or received.
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub kind: MessageKind,
    pub metadata: MetaData,
    pub content: Vec<Packet>,   // Vector of packets which together hold the whole content of Message.
    pub end_data: Packet,
}

impl Message {
    
    /// Creates a new Message, which is empty.
    /// Use other methods to fill it.
    pub fn new() -> Self {
        Message {
            kind: MessageKind::Unknown,
            metadata: MetaData::new_empty(),
            content: Vec::new(),
            end_data: Packet::new_empty(),
        }
    }

    /// This takes an ownership of self
    /// and sends a Message through given stream.
    pub fn send(self, stream: &mut TcpStream) {

        // Create a metadata_packet from metadata.
        let metadata_packet = Packet::new(PacketKind::new_metadata(self.metadata.clone()));

        // Write metadata_packet to stream.
        stream.write(&metadata_packet.to_buff()).unwrap();
        println!("metadata to buff: {:?}", &self.metadata.to_buff());
        
        // Write all content packets to stream.
        for packet in self.content {
            stream.write(&packet.clone().to_buff()).unwrap();
            println!("content to buff: {:?}", &packet.to_buff());  
        }
        
        // Write a end_data packet to stream.
        stream.write(&self.end_data.clone().to_buff()).unwrap();
    }

    /// Receives a Message from given stream.
    // USE RESULT AS RETURN TYPE.
    pub fn receive(stream: &mut TcpStream) -> Self {

        // Create new empty Message.
        let mut msg = Message::new();
        
        // Loop to read all packets.
        loop {
            // Read size of incoming packet.
            let mut size_buff = vec![0_u8; 8];
            stream.read_exact(&mut size_buff).unwrap();
            let size = usize::from_buff(size_buff.clone());

            // Read rest of packet.
            let mut buff = vec![0_u8; size - 8];
            stream.read_exact(&mut buff).unwrap();

            // Connect whole buffer.
            size_buff.extend(buff);
            let buff = size_buff;
            
            // Create a packet from buffer.
            let packet = Packet::from_buff(buff);

            // I would like change kind to private later.
            // Get a packet kind and modify msg based on that. 
            match packet.get_kind() {
                PacketKind::Empty => {
                    println!("Empty");
                },
                PacketKind::MetaData(..) => {
                    let metadata = MetaData::from_buff(packet.get_kind_owned().to_buff());
                    msg.set_metadata(metadata)
                    // msg.set_metadata(packet.kind.get_metadata().unwrap()); // CAN I USE UNWRAP?
                },
                PacketKind::Content(..) => {
                    msg.push_content(packet);
                },
                PacketKind::Request => {
                    println!("Request");
                },
                PacketKind::End => {
                    msg.set_end_data(packet);
                    break;
                },
                PacketKind::Unknown => {
                    println!("Unknown.")
                },
            }  
            // Just debug.
            println!("{:?}", &msg);
        }         

        msg
    }

    /// Sets a metadata of Message.
    /// Takes an ownership of metadata given in argument.
    pub fn set_metadata(&mut self, metadata: MetaData) {

        self.kind = metadata.message_kind.clone();
        self.metadata = metadata;
    }

    /// Adds a new Packet to content.
    /// Takes an ownership of packet given in argument. 
    pub fn push_content(&mut self, packet: Packet) {
        self.content.push(packet);
    }

    /// Sets an end_data of Message.
    /// Takes an ownership of end_data given in argument.
    pub fn set_end_data(&mut self, end_data: Packet) {
        self.end_data = end_data;
    }

    /// This takes an ownership of self
    /// and returns the whole content of all Packets as a single Vec<u8>.
    pub fn get_content(self) ->  Vec<u8> {
        let mut content: Vec<u8> = Vec::new();
        for data in self.content.into_iter() {
            if let PacketKind::Content(_, data) = data.get_kind_owned() { // Beware beware the unwrap comes.
                content.extend(data);
            }
        }
        content
    }
}