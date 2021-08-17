use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;

use serde::{Serialize, Deserialize};
use itertools::Itertools;

use crate::buffer::{ToBuffer, FromBuffer};
use crate::error::{NetCommsError, NetCommsErrorKind};
use crate::command::Command;
use crate::message::MessageKind;
use crate::packet::{MetaData, PacketKind, Packet};
use crate::config::{MAX_PACKET_CONTENT_SIZE, MAX_PACKET_SIZE, SERVER_ID, SERVER_NAME};
use crate::prelude::{Request, User, UserUnchecked};


/// Struct holds all information about message to be sent or received.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    kind: MessageKind,
    metadata: MetaData,
    // Vector of packets which together hold the whole content of Message.
    content: Vec<Packet>, // This should hold some info about file, if that is what was sent.
    end_data: Packet,
}

impl Message {
    
    /// Creates a new Message, which is empty.
    /// Use other methods to fill it.
    pub fn new() -> Result<Self, NetCommsError> {
        Ok(Message {
            kind: MessageKind::Unknown,
            metadata: MetaData::new_empty()?,
            content: Vec::new(),
            end_data: Packet::new_empty(),
        })
    }

    /// Creates a new message from Command.
    /// Messages created by this method can not be used by server as it allows multiple recipients, server always sends messages with only
    /// one recipient, or the caller has to made sure to enter only one recipient.
    pub fn from_command(command: Command) -> Result<Self, NetCommsError> {

        // As commands can be created only by client or in client like fashion, recipient_id will always be SERVER_ID.
        match command {
            Command::Send(message_kind, author, recipients, content, file_name) => {
                return Self::from_send(message_kind, author, recipients, content, file_name);    
            }
            Command::Register(user_unchecked, author) => {
                return Self::from_register(user_unchecked, author);
            }
            Command::Login(user_unchecked, author) => {
                return Self::from_login(user_unchecked, author);
            }
            _ => {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::WrongCommand,
                    Some("Message::from_command() failed to create a message from given command.".to_string())));
            }
        }
    }

    /// Creates Message from 'Command::Send'.
    fn from_send(message_kind: MessageKind,
                 author: User, recipients: Vec<String>,
                 content: Vec<u8>, file_name: Option<String>) -> Result<Self, NetCommsError> {

        let mut message = Self::new()?;

        let metadata = MetaData::new(&content, message_kind, author, SERVER_ID, recipients, file_name)?;
        message.set_metadata(metadata);

        let content = Self::split_to_max_packet_size(content);
        for buffer in content {
            message.push_content(Packet::new(PacketKind::new_content(buffer)))
        }

        let end_data = Packet::new(PacketKind::End);
        message.set_end_data(end_data);

        Ok(message)    
    }

    /// Creates a Message from 'Command::Register'.
    fn from_register(user_unchecked: UserUnchecked, author: User) -> Result<Self, NetCommsError> {

        let mut message = Self::new()?;

        let request = Request::Register(user_unchecked);
        let content = request.to_ron()?.to_buff()?;

        let message_kind = MessageKind::Request;
        let recipients = vec![SERVER_NAME.to_string().clone()];
        let file_name = None;

        let metadata = MetaData::new(&content, message_kind, author, SERVER_ID, recipients, file_name)?;
        message.set_metadata(metadata);

        let content = Self::split_to_max_packet_size(content);
        for buffer in content {
            message.push_content(Packet::new(PacketKind::new_content(buffer)))
        }
        let end_data = Packet::new(PacketKind::End);
        message.set_end_data(end_data);

        Ok(message)  
    }

    /// Creates a Message from 'Command::Login'.
    fn from_login(user_unchecked: UserUnchecked, author: User) -> Result<Self, NetCommsError> {

        let mut message = Self::new()?;

        let request = Request::Login(user_unchecked);
        let content = request.to_ron()?.to_buff()?;

        let message_kind = MessageKind::Request;
        let recipients = vec![SERVER_NAME.to_string().clone()];
        let file_name = None;

        let metadata = MetaData::new(&content, message_kind, author, SERVER_ID, recipients, file_name)?;
        message.set_metadata(metadata);

        let content = Self::split_to_max_packet_size(content);
        for buffer in content {
            message.push_content(Packet::new(PacketKind::new_content(buffer)))
        }
        let end_data = Packet::new(PacketKind::End);
        message.set_end_data(end_data);

        Ok(message)  
    }

    pub fn from_request(request: Request) {}
    /// This takes an ownership of self and sends a Message through given stream.
    pub fn send(self, stream: &mut TcpStream) -> Result<(), NetCommsError> {

        // Create multiple metadata packets if necessary and write them to stream.
        let metadata_buff = self.metadata.to_buff()?;
        let metadata_buff_split = Self::split_to_max_packet_size(metadata_buff);
        let metadata_length = metadata_buff_split.len();

        let mut id = 0;
        for buff in metadata_buff_split {
            id += 1;
            let packet: Packet;
            if id == metadata_length {
                packet = Packet::new(PacketKind::new_metadata_end(buff));
            } else {
                packet = Packet::new(PacketKind::new_metadata(buff));
            }
            if let Err(e) = stream.write_all(&packet.to_buff()?) {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::WritingToStreamFailed,
                    Some(format!("Failed to write a buffer to stream. ({})", e))));
            }
        }
                
        // Write all content packets to stream.
        for packet in self.content {
            if let Err(e) = stream.write_all(&packet.to_buff()?) {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::WritingToStreamFailed,
                    Some(format!("Failed to write a buffer to stream. ({})", e))));
            }
        }
        
        // Write an end_data packet to stream. If there ever will be added any real end data, this need to be reworked.
        if let Err(e) = stream.write_all(&self.end_data.to_buff()?) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::WritingToStreamFailed,
                Some(format!("Failed to write a buffer to stream. ({})", e))));
        }

        Ok(())
    }

    /// This takes an ownership of self and unlike text message this sends
    /// metadata first, then will gradually read the file to max packet length,
    /// sends the Packet and continue to the end of the file so there is no
    /// risk of overflowing the memory with too big files.
    pub fn send_file(self, stream: &mut TcpStream) -> Result<(), NetCommsError> {

        if let Some(file_name) = self.metadata.file_name() {
            match File::open(&file_name) {
                Ok(file) => {
                    let file_length = file.metadata().unwrap().len(); // Could not find in what case this returns an error, will check later if necessary.

                    let n_of_content_packets: usize;

                    if file_length as usize % MAX_PACKET_SIZE != 0 {
                        n_of_content_packets = file_length as usize / (MAX_PACKET_CONTENT_SIZE) + 1;
                    } else {
                        n_of_content_packets = file_length as usize / (MAX_PACKET_CONTENT_SIZE);
                    }

                    // This part can be optimized, by getting the length of metadata from its contents, without the need to convert it to buffer.
                    let metadata_buff = self.metadata.to_buff()?;
                    let n_of_metadata_packets = Self::number_of_packets(&metadata_buff);
                    
                    let n_of_packets = n_of_metadata_packets + n_of_content_packets + 1;

                    let mut metadata = MetaData::from_buff(metadata_buff)?;
                    metadata.set_message_length(n_of_packets);

                    let file_name: Option<String>;
                    match metadata.file_name() {
                        Some(path) => {
                            let path = Path::new(&path);
                            file_name = path.file_name().map(|name| name.to_string_lossy().into_owned());
                        },
                        None => return Err(NetCommsError::new(
                            NetCommsErrorKind::IncompleteMetaData,
                            Some("Missing file name while trying to send a file.".to_string()))),
                    }

                    metadata.set_file_name(file_name);

                    // Create multiple metadata packets if necessary and write them to stream.
                    let metadata_buff = metadata.to_buff()?;
                    let metadata_buff_split = Self::split_to_max_packet_size(metadata_buff);
                    
                    let mut id = 0;
                    let n_of_mtd = metadata_buff_split.len();
                    for buff in metadata_buff_split {
                        id += 1;
                        let packet: Packet;
                        if id == n_of_mtd {
                            packet = Packet::new(PacketKind::new_metadata_end(buff));
                        } else {
                            packet = Packet::new(PacketKind::new_metadata(buff));
                        }
                        
                        if let Err(e) = stream.write(&packet.to_buff()?) {
                            return Err(NetCommsError::new(
                                NetCommsErrorKind::WritingToStreamFailed,
                                Some(format!("Failed to write metadata packet to stream. ({})", e))));
                        }
                    }

                    let mut reader = BufReader::new(file);
                    for part in 1..=n_of_content_packets {
                        let packet: Packet;
                        {
                            let mut buff: Vec<u8>;   
                            if part == n_of_content_packets {
                                buff = Vec::new();
                                if let Err(e) = reader.read_to_end(&mut buff) {
                                    return Err(NetCommsError::new(
                                        NetCommsErrorKind::ReadingFromFileFailed,
                                        Some(format!("Failed to read last content packet from file. ({})", e))));
                                }
                            } else {
                                buff = vec![0_u8; MAX_PACKET_CONTENT_SIZE];
                                if let Err(e) = reader.read_exact(&mut buff) {  // This read_exact instead of read is really important.
                                    return Err(NetCommsError::new(
                                        NetCommsErrorKind::ReadingFromFileFailed,
                                        Some(format!("Failed to read content packet from file. ({})", e))));
                                } 
                            }    

                            packet = Packet::new(PacketKind::new_content(buff));
                        }

                        if let Err(e) = stream.write(&packet.to_buff()?) {
                            return Err(NetCommsError::new(
                                NetCommsErrorKind::WritingToStreamFailed,
                                Some(format!("Failed to write content packet to stream. ({})", e))));
                        }
                    }

                    if let Err(e) = stream.write(&self.end_data.clone().to_buff()?) {
                        return Err(NetCommsError::new(
                            NetCommsErrorKind::WritingToStreamFailed,
                            Some(format!("Failed to write end data packet to stream. ({})", e))));
                    }  

                    Ok(())
                }
                Err(e) => return Err(NetCommsError::new(
                    NetCommsErrorKind::OpeningFileFailed,
                    Some(format!("Opening a file {} failed. ({})", file_name, e)))),
            }
        } else {
            Err(NetCommsError::new(
                NetCommsErrorKind::IncompleteMetaData,
                Some("File can not be sent, missing file name".to_string())))
        }            
    }

    /// Receives a Message from given stream.
    // USE RESULT AS RETURN TYPE.

    // Needs to be refactored. Some errors probably should be handled right in the function = not so many question marks. Get rid of unwraps, where needed.

    pub fn receive(stream: &mut TcpStream) -> Result<Self, NetCommsError> {

        // Create new empty Message.
        let mut msg = Message::new()?;
        let mut metadata_buff = Vec::new(); 

        loop {

            // Read size of packet.
            let mut size_buff = vec![0_u8; 8];
            if let Err(e) = stream.read_exact(&mut size_buff) {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::ReadingFromStreamFailed, 
                    Some(format!("Failed to read the size of metadata packet. ({})", e))));
            }
            let size = usize::from_buff(size_buff.clone())?;

            // Read rest of packet.
            let mut buff = vec![0_u8; size - 8];    // - 8 for size of packet encoded as bytes which already exist.
            if let Err(e) = stream.read_exact(&mut buff) {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::ReadingFromStreamFailed, 
                    Some(format!("Failed to read the contents of metadata packet. ({})", e))));
            }

            // Connect whole buffer.
            size_buff.extend(buff);
            let buff = size_buff;
            
            // Create a packet from buffer.
            let packet = Packet::from_buff(buff)?;
            
            match packet.kind() {
                PacketKind::MetaData(..) => {
                    metadata_buff.extend(packet.kind_owned().content()?);
                }
                PacketKind::MetaDataEnd(..) => {
                    metadata_buff.extend(packet.kind_owned().content()?);
                    break;
                },
                _ => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::InvalidPacketKind, 
                        Some(format!("Unexpected PacketKind, expected MetaData or MetaDataEnd, arrived:\n {:?}", packet.kind_owned()))));
                }               
            }
        }

        msg.set_metadata(MetaData::from_buff(metadata_buff)?);
  
        // Loop to read all content packets.
        loop {
            // Read size of incoming packet.
            let mut size_buff = vec![0_u8; 8];
            if let Err(e) = stream.read_exact(&mut size_buff) {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::ReadingFromStreamFailed, 
                    Some(format!("Failed to read the size of content packet. ({})", e))));
            }
            let size = usize::from_buff(size_buff.clone())?;

            // Read rest of packet.
            let mut buff = vec![0_u8; size - 8];    // - 8 for size of packet encoded as bytes which already exist.
            if let Err(e) = stream.read_exact(&mut buff) {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::ReadingFromStreamFailed, 
                    Some(format!("Failed to read the contents of content packet. ({})", e))));
            }

            // Connect whole buffer.
            size_buff.extend(buff);
            let buff = size_buff;
            
            // Create a packet from buffer.
            let packet = Packet::from_buff(buff)?;

            // Get a packet kind and modify msg based on that. 
            match packet.kind() {
                PacketKind::Empty => {
                    println!("Empty");
                },
                PacketKind::MetaData(..) => {
                    println!("MetaData");
                },
                PacketKind::MetaDataEnd(..) => {
                    println!("MetaDataEnd");
                }
                PacketKind::Content(..) => {
                    match msg.metadata().message_kind() {
                        MessageKind::File => {
                            // Cases where the file with the same name that already exists is send, should be solved,
                            // as for no it just appends incoming content to the current.
                            let mut file = OpenOptions::new()
                                                            .create(true)
                                                            .append(true)
                                                            .write(true)
                                                            .open(msg.metadata().file_name().unwrap()) //Unwrap can be used.
                                                            .unwrap();

                            if let Err(e) = file.write(&packet.kind_owned().content()?) {
                                return Err(NetCommsError::new(
                                    NetCommsErrorKind::WritingToFileFailed,
                                    Some(format!("Could not write to file. ({})", e))));
                            }                    
                        }
                        _ => {
                            msg.push_content(packet);
                        }
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
        Ok(msg)
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

    /// Returns a MessageKind
    pub fn kind(&self) -> MessageKind {
        self.kind.clone()
    }

    // Returns Message metadata.
    pub fn metadata(&self) -> MetaData {
        self.metadata.clone()
    }

    /// This takes an ownership of self
    /// and returns the whole content of all Packets as a single Vec<u8>.
    pub fn content(self) ->  Vec<u8> {
        let mut content: Vec<u8> = Vec::new();
        for data in self.content.into_iter() {
            if let PacketKind::Content(_, data) = data.kind_owned() { 
                content.extend(data);
            }
        }
        content
    }

    /// Returns a number of packets that will need to be created from given buffer.
    pub fn number_of_packets(content: &Vec<u8>) -> usize {

        let byte_length = content.len();

        // Get number of packets by dividing by MAX_PACKET_CONTENT_SIZE.
        let mut number_of_packets = byte_length / MAX_PACKET_CONTENT_SIZE;  
        // Add one packet if there is any remainder after the division.
        if byte_length % MAX_PACKET_CONTENT_SIZE != 0 {
            number_of_packets += 1;
        }
        number_of_packets
    }

    /// Splits given buffer to vector of buffers with size of MAXIMUM_PACKET_SIZE.
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
}
