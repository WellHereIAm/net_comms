use std::path::{Path, PathBuf};
use std::fs::OpenOptions;

use library::{buffer::IntoBuffer, message::{ContentType}, packet::Packet, prelude::{IntoRon, Message, MetaDataType, NetCommsError}};

use super::{message_kind::MessageKind, metadata::MetaData};

pub struct Content (String);

impl ContentType<'a, MessageKind, MetaData, Content> for Content {
    
    fn send(self, stream: &mut TcpStream, metadata: MetaData) -> Result<(), NetCommsError> {

        match metadata.file_name() {
            Some(file_name) => {
                let path = Path::new(file_name);
                self.send_file(stream, path)?;
            }
            None => self.send_content(stream)?,
        }

        Ok(())
    }

    fn receive(stream: &mut TcpStream,
               message: &mut Message<'a, MessageKind, MetaData, Content>,
               location: Option<PathBuf>) -> Result<(Self, Packet), NetCommsError> {

        let location = location.unwrap();
        let location = message.metadata().get_message_location(location);

        if let Err(e) = fs::create_dir_all(&location) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::CreatingDirFailed,
                Some(format!("Could not create a directory on {}. \n({})",
                                     &location.parent().unwrap().to_str().unwrap(), e))));
        }

        let (content, end_data) = match message.metadata().message_kind() {
            MessageKind::File => {
                let end_data = self.receive_file(stream, location)?;
                (Content::new(), end_data)
            }
            _ => {
                let (content, end_data) = self.receive_content()?;
                (content, end_data)
            }
        };

        message.set_content(content.clone());
        message.set_end_data(end_data.clone());

        let message_ron = message.into_ron()?;
        location.push("message.ron");
        let mut file = fs::OpenOptions::new().create(true).write(true).open(location).unwrap();
        file.write_fmt(format_args!("{}", message_ron)).unwrap();

        Ok(Content::new(), end_data)
    }
}


impl Content {

    pub fn new() -> Self {
        Content(String::new())
    }

    pub fn with_data(data: String) -> Self {
        Content(data)
    }
    
    pub fn append_string(&mut self, string: String) {
        for char in string.chars() {
            self.0.push(char);
        }
    }
    fn send_content(self, stream: &mut TcpStream) -> Result<(), NetCommsError> {

        let content = self.0.into_buff()?;
        let content_split = Packet::split_to_max_packet_size(content);

        // Write all content packets to stream.
        for packet_content in content_split.into_iter() {
            let packet = Packet::new(PacketKind::Content, packet_content);
            if let Err(e) = stream.write_all(&packet.into_buff()?) {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::WritingToStreamFailed,
                    Some(format!("Failed to write a buffer to stream. ({})", e))));
            }
        }

        Ok(())
    }

    fn send_file(self, stream: &mut TcpStream, path: &Path) -> Result<(), NetCommsError> {

        let file = match File::open(path.file_name()) {
            Ok(file) => file,
            Err(e) => return Err(NetCommsError::new(
                NetCommsErrorKind::OpeningFileFailed,
                Some(format!("Opening a file {} failed. ({})", file_name, e)))),
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
            {
                let mut buff: Vec<u8>;   
                if part == n_of_content_packets {
                    buff = Vec::new();
                    if let Err(e) = reader.read_to_end(&mut buff) {
                        return Err(NetCommsError::new(
                            NetCommsErrorKind::ReadingFromFileFailed,
                            Some(format!("Failed to read last content packet from file. ({})",
                                         e))));
                    }
                } else {
                    // Create a buffer with exact buffer size.
                    buff = vec![0_u8; MAX_PACKET_CONTENT_SIZE as usize];
                    // This read_exact instead of read is really important.
                    if let Err(e) = reader.read_exact(&mut buff) {  
                        return Err(NetCommsError::new(
                            NetCommsErrorKind::ReadingFromFileFailed,
                            Some(format!("Failed to read content packet from file. ({})", e))));
                    } 
                }    

                packet = Packet::new(PacketKind::Content, buff);
            }

            packet.send(stream)?;
        }

        Ok(()) 
    }

    fn receive_content(stream: &mut TcpStream) -> Result<(Self, Packet), NetCommsError> {

        let mut content = Content::new();
        
        loop {
            let packet = Packet::receive(stream)?;

            match packet.kind() {
                PacketKind::Empty => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::InvalidPacketKind, 
                        Some("Expected content packet, got empty.")));
                },
                PacketKind::MetaData
                | PacketKind::MetaDataEnd => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::InvalidPacketKind, 
                        Some("Expected content packet, got metadata.")));
                },
                PacketKind::Content => {
                    content.append_string();
                },
                PacketKind::End => {
                    let end_data = packet;
                    return (content, end_data);
                },
                PacketKind::Unknown => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::UnknownMessageKind,
                        None))
                },
            } 
        }        
    }

    fn receive_file(stream: &mut TcpStream,
                    message: &mut Message<'a, MessageKind, MetaData, Content>,
                    location: PathBuf) -> Result<Packet, NetCommsError> {
        
        let location = location.push(message.metadata().file_name().unwrap());
        let create_file_result = OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open(location);
        
        let file = match create_file_result {
            Ok(file_in_result) => file = file_in_result,
            Err(e) => {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::CreatingFileFailed,
                    Some(format!("Could not create a file: {}. ({})", message.metadata().file_name().unwrap(), e))));
            }
        };

        loop {
            let packet = Packet::receive(stream)?;

            match packet.kind() {
                PacketKind::Empty => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::InvalidPacketKind, 
                        Some("Expected content packet, got empty.")));
                },
                PacketKind::MetaData
                | PacketKind::MetaDataEnd => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::InvalidPacketKind, 
                        Some("Expected content packet, got metadata.")));
                },
                PacketKind::Content => {    
                    // Write to file.
                    if let Err(e) = file.write(&packet.content_move()) {
                        return Err(NetCommsError::new(
                            NetCommsErrorKind::WritingToFileFailed,
                            Some(format!("Could not write to file. ({})", e))));
                    }    
                },
                PacketKind::End => {
                    let end_data = packet;
                    return (Content::new(), end_data);
                },
                PacketKind::Unknown => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::UnknownMessageKind,
                        None))
                },
            }

        }
    }
}
