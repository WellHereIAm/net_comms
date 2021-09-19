use serde::{Serialize, Deserialize};

use std::fmt::{Debug, Display};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};

use crate::bytes::{Bytes, IntoBytes};
use crate::error::{NetCommsError, NetCommsErrorKind};
use crate::packet::{Packet, PacketKind};
use crate::ron::{FromRon, ToRon};
use crate::message::{ContentType, MetaDataType};

/// Default struct to store data sent or received.
///
/// # Fields
/// * `metadata` -- has to implement [MetaDataType].
/// * `content` -- has to implement [ContentType].
/// * `end_data` -- marks end of [Message] during transmission, but can also hold some data.
///
/// Usually is used by declaring custom [type].
/// ```
/// type MyMessage = Message<MyMetaData, MyContent>;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<M, C> {
    metadata: M,
    content: C, 
    end_data: Packet,
}

impl<M, C> Default for Message<M, C> 
where
    M: Default,
    C: Default {
    
    fn default() -> Self {
        Message {
            metadata: M::default(),
            content: C::default(),
            end_data: Packet::default(),
        }
    }
}

impl<M, C> Display for Message<M, C>
where
    M: Serialize,
    C: Serialize {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let formatted = self.to_ron_pretty(None).expect("Failed to parse a Message to RON.");
        write!(f, "{}", &formatted)
    }
}

impl<'a, M, C> ToRon for Message<M, C> 
where
    M: Serialize,
    C: Serialize {}

impl<'a, M, C> FromRon<'a> for Message<M, C> 
where
    M: Deserialize<'a>,
    C: Deserialize<'a> {}


    
impl<'a, M, C> Message<M, C> 
where
    M: Default + Clone + MetaDataType<'a> + Debug,
    C: Default + Clone + ContentType<'a, M, C> + Debug {

    /// Creates an empty [Message].
    /// 
    /// Usually created as mutable and later given data.
    ///
    /// ```
    /// use library::packet::{Packet, PacketKind};
    ///
    /// type MyMessage = Message<MyMetaData, MyContent>;
    //
    /// let mut message = MyMessage::new();
    /// 
    /// message.set_metadata(MyMetaData::new());
    /// message.set_content(MyContent::new());
    /// message.set_end_data(Packet::new(PacketKind::End, Bytes::new()));
    /// ```
    /// * Note: This example will not work as referenced structs do not exist.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Sends `Self` via [`stream`](std::net::TcpStream).
    ///
    /// Takes an ownership of `self`.
    ///
    /// This uses send methods implemented by `metadata` and `content`.
    ///
    /// This will return [Ok] with an empty [tuple] inside if succeeds to send `self`, otherwise returns [NetCommsError].
    pub fn send(self, stream: &mut TcpStream) -> Result<(), NetCommsError> {
        
        // MetaDataType needs to return valid metadata, if they need to be used while sending content,
        // otherwise it can just return an empty, invalid metadata.
        let metadata = self.metadata.send(stream)?;
        self.content.send(stream, metadata)?;
        self.end_data.send(stream)?;

        Ok(())
    }

    /// Receives a [Message] from [`stream`](std::net::TcpStream).
    ///
    /// * `location` -- an optional path to location on the device, that can be used to store 
    /// received data inside methods implemented by `metadata` and `content`.
    pub fn receive(stream: &mut TcpStream, location: Option<PathBuf>) -> Result<Self, NetCommsError> {
        
        let mut message = Self::default();

        let metadata = M::receive(stream, location.clone())?;
        // Since end of content is marked by end_data packet, it needs to be returned with content.
        let (content, end_data) = C::receive(stream, &metadata, location)?;

        message.set_metadata(metadata);
        message.set_content(content);
        message.set_end_data(end_data);

        Ok(message)
    }

    /// Sends [`content`](Bytes) via `stream`.
    ///
    /// This method can be used in implementation of [ContentType].
    pub fn send_content(stream: &mut TcpStream, content: Bytes) -> Result<(), NetCommsError>  {

        let content_split = Packet::split_to_max_packet_size(content);

        // Write all content packets to stream.
        for packet_content in content_split.into_iter() {
            let packet = Packet::new(PacketKind::Content, packet_content);
            packet.send(stream)?;
        }

        Ok(())
    }

    /// Receives [`content`](Bytes) and `end_data` [Packet] from `stream`.
    ///
    /// This method can be used in implementation of [ContentType].
    pub fn receive_content(stream: &mut TcpStream) -> Result<(Bytes, Packet), NetCommsError> {

        let mut content = Bytes::new();
        
        // loop until end_data packet is received.
        loop {
            let mut packet = Packet::receive(stream)?;

            match packet.kind() {
                PacketKind::Empty => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::InvalidPacketKind, 
                        Some("Expected content packet, got empty.".to_string())));
                },
                PacketKind::MetaData
                | PacketKind::MetaDataEnd => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::InvalidPacketKind, 
                        Some("Expected content packet, got metadata.".to_string())));
                },
                PacketKind::Content => {
                    content.append(packet.content_mut());
                },
                PacketKind::End => {
                    let end_data = packet;
                    return Ok((content, end_data));
                },
                PacketKind::Unknown => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::UnknownPacketKind,
                        None))
                },
            } 
        }
    }

    /// Sends a file from [`path`](Path) via a [`stream`](TcpStream).
    ///
    /// This method differs from other send methods as this does not load whole file into memory,
    /// but it gradually reads file sends part with size equal to [MAX_CONTENT_SIZE](Packet::max_content_size).
    pub fn send_file(stream: &mut TcpStream, path: &Path) -> Result<(), NetCommsError> {

        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(NetCommsError::new(
                NetCommsErrorKind::OpeningFileFailed,
                Some(format!("{}", e)))),
        };

        // Get information about to how many packet the file will be split.
        let n_of_content_packets = Packet::number_of_packets(file
                                                .metadata()
                                                .unwrap()
                                                .len() as usize);
        
        let mut reader = BufReader::new(file);

        // Starts at 1 and ends inclusively at n_of_content_packets so the whole file is read.
        for part in 1..=n_of_content_packets {
            let packet: Packet;
            
            let mut buff: Vec<u8>;   
            if part == n_of_content_packets {
                // Read last part of file.
                buff = Vec::new();
                if let Err(e) = reader.read_to_end(&mut buff) {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::ReadingFromFileFailed,
                        Some(format!("Failed to read last content packet from file.\n({})",
                                     e))));
                }
            } else {
                // Create a buffer with exact buffer size.
                buff = vec![0_u8; Packet::max_content_size() as usize];
                // This read_exact instead of read is really important.
                if let Err(e) = reader.read_exact(&mut buff) {  
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::ReadingFromFileFailed,
                        Some(format!("Failed to read content packet from file.\n({})", e))));
                } 
            }    

            packet = Packet::new(PacketKind::Content, buff.into_bytes());
            
            packet.send(stream)?;
        }

        Ok(())
    }

    /// Receives a file from [`stream`](TcpStream) and saves it to [`location`](Path) with `file_name`.
    ///
    /// This method differs from other receive methods as this does not load whole file into memory,
    /// but it gradually receives part of file and saves it.
    pub fn receive_file(stream: &mut TcpStream,
                        location: &Path, file_name: String,
                        ) -> Result<(usize, Packet), NetCommsError> {

        let mut path = location.to_path_buf();
        path.push(&file_name);

        let file = match File::create(path) {
            Ok(file) => file,
            Err(e) => {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::CreatingFileFailed,
                    Some(format!("Could not create a file: {}.\n({})", file_name, e))));
            },
        };

        let mut writer = BufWriter::new(file);

        let mut number_of_packets = 0;
        // Loop to receive packets until end_data packet comes.
        loop {
            let packet = Packet::receive(stream)?;

            match packet.kind() {
                PacketKind::Empty => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::InvalidPacketKind, 
                        Some("Expected content packet, got empty.".to_string())));
                },
                PacketKind::MetaData
                | PacketKind::MetaDataEnd => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::InvalidPacketKind, 
                        Some("Expected content packet, got metadata.".to_string())));
                },
                PacketKind::Content => {    
                    // Write to file.
                    if let Err(e) = writer.write(&packet.content_move().into_vec()) {
                        return Err(NetCommsError::new(
                            NetCommsErrorKind::WritingToFileFailed,
                            Some(format!("Could not write to file. ({})", e))));
                    }    
                },
                PacketKind::End => {
                    writer.flush().unwrap();
                    let end_data = packet;
                    return Ok((number_of_packets, end_data));
                },
                PacketKind::Unknown => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::UnknownMessageKind,
                        None))
                },
            }

            number_of_packets += 1;
        }  
    }
    
    /// Saves `Self` into `location`.
    pub fn save(&self, location: &Path) {

        let message_ron = self.to_ron().unwrap();
        fs::create_dir_all(location.parent().unwrap()).unwrap();

        let mut file = fs::OpenOptions::new().create(true).write(true).open(location).unwrap();
        file.write_fmt(format_args!("{}", message_ron)).unwrap();
    }

    // Implementation of setters and getters for Message.

    /// Returns `metadata`.
    ///
    /// Metadata are cloned.
    pub fn metadata(&self) -> M {
        self.metadata.clone()
    }

    /// Returns a reference to `metadata`.
    pub fn metadata_ref(&'a self) -> &'a M {
        &self.metadata
    }

    /// Returns a mutable reference to `metadata`.
    pub fn metadata_mut(&'a mut self) -> &'a mut M {
        &mut self.metadata
    }

    /// Takes ownership of `Self` and return owned `metadata`.
    pub fn metadata_move(self) -> M {
        self.metadata
    }

    pub fn content(&self) -> C {
        self.content.clone()
    }

    /// Return a reference to `content`.
    pub fn content_ref<'b>(&'b self) -> &'b C {
        &self.content
    }

    /// Return a mutable reference to `content`.
    pub fn content_mut<'b>(&'b mut self) -> &'b mut C {
        &mut self.content
    }

    /// Takes ownership of `Self` and return owned `content`.
    pub fn content_move(self) -> C {
        self.content
    }
    
    /// Returns `end_data`.
    ///
    /// `end_data` are cloned.
    pub fn end_data(&self) -> Packet {
        self.end_data.clone()
    }

    /// Sets [Message] `metadata` to given.
    pub fn set_metadata(&mut self, metadata: M) {
        self.metadata = metadata;
    }

    /// Sets [Message] `content` to given.
    pub fn set_content(&mut self, content: C) {
        self.content = content;
    }

    /// Sets [Message] `end_data` to given packet, this [Packet] should have `kind` [PacketKind::End].
    pub fn set_end_data(&mut self, end_data: Packet) {
        self.end_data = end_data;
    }
}