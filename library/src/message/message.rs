use std::fs::{self, File, OpenOptions, read};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::net::TcpStream;
use std::path::Path;

use serde::{Serialize, Deserialize};
use itertools::Itertools;

use crate::buffer::{ToBuffer, FromBuffer};
use crate::error::{NetCommsError, NetCommsErrorKind};
use crate::command::Command;
use crate::message::MessageKind;
use crate::packet::{MetaData, PacketKind, Packet};
use crate::config::{MAX_PACKET_SIZE, SERVER_ID};


/// Struct holds all information about message to be sent or received.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    kind: MessageKind,
    metadata: MetaData,
    content: Vec<Packet>,   // Vector of packets which together hold the whole content of Message.
    end_data: Packet,
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

    /// Creates a new message from Command.
    /// This method can be used only by client as it allows multiple recipients, server is always creating messages with one recipient.
    // Result is probably unnecessary as this will be called by send variant itself? Change after command processing.
    pub fn from_command(command: Command) -> Result<Self, NetCommsError> {

        match command {
            Command::Send(msg_kind, author_id, recipients, content, file_name) => {

                let mut msg = Self::new();

                let vectored_content = Self::split_to_max_packet_size(content);

                // Get number of content packets.
                let mut n_of_content_packets = 0;
                for vec in vectored_content.into_iter() {
                    n_of_content_packets += 1;
                    let packet = Packet::new(PacketKind::new_content(vec));
                    msg.push_content(packet);
                }

                let temp_metadata = MetaData::new(msg_kind.clone(), 0,
                                                          author_id,
                                                          SERVER_ID, recipients.clone(),
                                                          file_name.clone());
                                                      
                let n_of_metadata_packets = Self::split_to_max_packet_size(temp_metadata.to_buff()).len();

                // Adds number of MetaData packets to number of Content packets to one End packet.
                let msg_length = n_of_metadata_packets + n_of_content_packets + 1; 

                let metadata = MetaData::new(msg_kind, msg_length, author_id, SERVER_ID, recipients, file_name);
                

                msg.set_metadata(metadata);
                msg.set_end_data(Packet::new(PacketKind::End));

                Ok(msg)
            },
            _ => Err(NetCommsError::new(NetCommsErrorKind::WrongCommand))
        }
    }

    /// This takes an ownership of self
    /// and sends a Message through given stream.
    pub fn send(self, stream: &mut TcpStream) {

        // Crates multiple metadata packets if necessary and writes them to stream.
        let metadata_buff = self.metadata.to_buff();
        let metadata_buff_split = Self::split_to_max_packet_size(metadata_buff);

        for buff in metadata_buff_split {
            let packet = Packet::new(PacketKind::new_metadata(buff));
            stream.write(&packet.to_buff()).unwrap();
        }
                
        // Write all content packets to stream.
        for packet in self.content {
            stream.write(&packet.to_buff()).unwrap();
        }
        
        // Write a end_data packet to stream.
        stream.write(&self.end_data.clone().to_buff()).unwrap();
    }

    /// This takes an ownership of self and unlike text message this sends
    /// metadata first, then will gradually read the file to max packet length,
    /// sends the Packet and continue to the end of the file so there is no
    /// risk of overflowing the memory with too big files.
    pub fn send_file(self, stream: &mut TcpStream) {

        let file = fs::File::open(self.metadata.file_name().unwrap()).unwrap();
        let file_length = file.metadata().unwrap().len();

        let mut n_of_packets = 0;
        let n_of_content_packets: usize;

        if file_length as usize % MAX_PACKET_SIZE != 0 {
            n_of_content_packets = file_length as usize / (MAX_PACKET_SIZE - 10) + 1;
        } else {
            n_of_content_packets = file_length as usize / (MAX_PACKET_SIZE - 10);
        }

        // This part can be optimized, but this should work.
        let metadata_buff = self.metadata.to_buff();
        let n_of_metadata_packets = Self::split_to_max_packet_size(metadata_buff.clone()).len();

        n_of_packets += n_of_metadata_packets;
        n_of_packets += n_of_content_packets;
        n_of_packets += 1;

        let mut metadata = MetaData::from_buff(metadata_buff);
        metadata.set_message_length(n_of_packets);

        // Yes... 
        let file_name = Some(Path::new(&metadata
                                                .file_name()
                                                .unwrap())
                                                .file_name()
                                                .map(|name| name.to_string_lossy()
                                                .into_owned())
                                                .unwrap());

        metadata.set_file_name(file_name);

        // Crates multiple metadata packets if necessary and writes them to stream.
        let metadata_buff = metadata.to_buff();
        let metadata_buff_split = Self::split_to_max_packet_size(metadata_buff);

        for buff in metadata_buff_split {
            let packet = Packet::new(PacketKind::new_metadata(buff));
            stream.write(&packet.to_buff()).unwrap();
        }

        let mut reader = BufReader::new(file);

        for part in 1..=n_of_content_packets {
            let packet: Packet;
            {
                let mut buff: Vec<u8>;   
                if part == n_of_content_packets {
                    buff = Vec::new();
                    reader.read_to_end(&mut buff).unwrap();
                } else {
                    buff = vec![0_u8; MAX_PACKET_SIZE - 10];
                    reader.read(&mut buff).unwrap();
                }    

                packet = Packet::new(PacketKind::new_content(buff));
            }
            stream.write(&packet.to_buff()).unwrap();
        }

        stream.write(&self.end_data.clone().to_buff()).unwrap();  
    }

    /// Receives a Message from given stream.
    // USE RESULT AS RETURN TYPE.
    pub fn receive(stream: &mut TcpStream) -> Self {

    
        // Now is this function really ugly. Rewrite it. Separate to multiple functions.
        // I should revwrite this as two separate loops, one for metadata other for content

        // Create new empty Message.
        let mut msg = Message::new();
        let mut metadata_buff = Vec::new(); 

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

            match packet.kind() {
                PacketKind::MetaData(..) => {
                    metadata_buff.extend(packet.kind_owned().content().unwrap());
                }
                _ => break,                
            }
        }

        msg.set_metadata(MetaData::from_buff(metadata_buff));
        
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

            // Get a packet kind and modify msg based on that. 
            match packet.kind() {
                PacketKind::Empty => {
                    println!("Empty");
                },
                PacketKind::MetaData(..) => {
                    println!("MetaData");
                },
                PacketKind::Content(..) => {
                    // println!("MessageKind: {:?}", msg.metadata().message_kind());
                    if let MessageKind::File = msg.metadata().message_kind() {
                        // println!("Path: {}", msg.metadata().file_name().unwrap());
                        // Totally normal thing to have.
                        let mut file = OpenOptions::new()
                                                        .create(true)
                                                        .append(true)
                                                        .open(msg.metadata().file_name().unwrap())
                                                        .unwrap();
                        // println!("file: {:?}", &file);
                        file.write(&packet.kind_owned().content().unwrap()).unwrap();
                    } else {
                        println!("It´s else.");
                        msg.push_content(packet);
                    }
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
        }

        msg
    }

    /// Sets a metadata of Message.
    /// Takes an ownership of metadata given in argument.
    pub fn set_metadata(&mut self, metadata: MetaData) {

        self.kind = metadata.message_kind();
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
    pub fn content(self) ->  Vec<u8> {
        let mut content: Vec<u8> = Vec::new();
        for data in self.content.into_iter() {
            if let PacketKind::Content(_, data) = data.kind_owned() { // Beware beware the unwrap comes.
                content.extend(data);
            }
        }
        content
    }

    /// Splits given buffer to vector of buffer of MAXIMUM_PACKET_SIZE.
    pub fn split_to_max_packet_size(buffer: Vec<u8>) -> Vec<Vec<u8>> {

        // This splits given buffer to multiple owned chunks with chunks method from itertools crate,
        // then it will split every chunk to iterator as well which are then collected to vectors of bytes,
        // that are collected to single vector. 
        // This is not my work: https://stackoverflow.com/a/67009164. 
        let vectored_content: Vec<Vec<u8>> = buffer.into_iter()
                                                    .chunks(MAX_PACKET_SIZE - 10)
                                                    .into_iter()
                                                    .map(|chunk| chunk.collect())
                                                    .collect();

        vectored_content
    }

    /// Returns a MessageKind
    pub fn kind(&self) -> MessageKind {
        self.kind.clone()
    }

    pub fn metadata(&self) -> MetaData {
        self.metadata.clone()
    }
}