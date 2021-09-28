use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime, Utc};

use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use nardol::bytes::{Bytes, FromBytes, IntoBytes};
use nardol::error::{NetCommsError, NetCommsErrorKind};
use nardol::message::MetaDataType;
use nardol::ron::{ToRon, FromRon};
use nardol::packet::{Packet, PacketKind};

use super::message_kind::MessageKind;
use crate::user::{User, UserLite};


/// This struct holds metadata of each [Message].
///
/// # Fields
///
/// * `message_kind` -- [MessageKind] of [Message].
/// * `message_length` -- number of [packets](super::Packet) that needs to be send to accommodate whole [Message].
/// * `datetime` -- [DateTime<Utc>] of when was [Message] created, encode to [Vec] of [u8] to ease [serde] serializing and deserializing
/// * `author_id` -- id of client or [server](crate::config::SERVER_ID) if resend by server, so the id of author
/// will never reach recipient.
/// * `author_username`
/// * `recipient_id` -- if [Message] was created by client it is [SERVER_ID](crate::config::SERVER_ID), not ids of recipients, that 
/// is assigned later by server.
/// * `recipients` -- [Vec] of usernames of recipients.
/// * `file_name` -- [Option], if [Some] [MessageKind] is [File](MessageKind::File) and [String] inside holds a file name and
/// file extension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    message_kind: MessageKind,
    message_length: u32,  
    datetime: Bytes,  
    author_id: u32,   
    author_username: String,
    recipient_id: u32, 
    recipients: Vec<String>,
    file_name: Option<String>,  
}

impl Default for MetaData {
    
    fn default() -> Self {

        let datetime = Self::current_datetime().into_bytes();

        MetaData {
            message_kind: MessageKind::Empty,
            message_length: 0,
            datetime,
            author_id: 0,
            author_username: "".to_string(),
            recipient_id: 0,
            recipients: vec![],
            file_name: None,
        }
    }
}

impl ToRon for MetaData {}
impl FromRon<'_> for MetaData {}

impl IntoBytes for MetaData {

    /// This takes an ownership of self and first encodes MetaData to RON format which is then encoded to buffer.
    fn into_bytes(self) -> Bytes {
        IntoBytes::into_bytes(self.to_ron().unwrap())
    }
}

impl FromBytes for MetaData {

    /// Wrapper around MetaData::from_ron(), which takes String to return MetaData.
    ///
    /// Takes an ownership of buff and returns MetaData.
    /// This implementation does not check if the buffer has correct length as it can vary.
    fn from_bytes(bytes: Bytes) -> Result<Self, NetCommsError>
    where
            Self: Sized {

        let metadata = MetaData::from_ron(&String::from_bytes(bytes)?)?;
        Ok(metadata)
    }

    fn from_buff(buff: &[u8]) -> Result<Self, NetCommsError>
    where
            Self: Sized {
        
        let metadata = MetaData::from_ron(&String::from_buff(buff)?)?;
        Ok(metadata)
    }
}

impl MetaDataType<'_> for MetaData {
    
    fn send(self, stream: &mut std::net::TcpStream) -> Result<MetaData, NetCommsError> {

        // Create multiple metadata packets if necessary and write them to stream.
        let metadata_buff = self.into_buff();
        let metadata_buff_split = Packet::split_to_max_packet_size(metadata_buff.into_bytes());

        let mut metadata = Bytes::new();
        
        // id is used to know when end of metadata is reached, so MetaDataEnd can be send.
        let mut id = 0;    
        let n_of_metadata_packets = metadata_buff_split.len();

        for buff in metadata_buff_split {
            id += 1;
            let packet: Packet;
            if id == n_of_metadata_packets {
                packet = Packet::new(PacketKind::MetaDataEnd, buff);
            } else {
                packet = Packet::new(PacketKind::MetaData, buff);
            }

            let packet_buff = packet.into_buff();
            
            if let Err(e) = stream.write(&packet_buff) {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::WritingToStreamFailed,
                    Some(format!("Failed to write metadata packet to stream. ({})", e))));
            }

            let mut packet = Packet::from_buff(&packet_buff)?;
            metadata.append(packet.content_mut());
        }

        let metadata = MetaData::from_bytes(metadata)?;

        Ok(metadata)
    }

    fn receive(stream: &mut std::net::TcpStream, location: Option<PathBuf>) -> Result<Self, NetCommsError> {
        
        let mut metadata = Bytes::new(); 

        // Loop to read all metadata packets.
        loop {   
            let mut packet = Packet::receive(stream)?;         
            match packet.kind() {
                PacketKind::MetaData => {
                    metadata.append(packet.content_mut());
                }
                PacketKind::MetaDataEnd => {
                    metadata.append(packet.content_mut());
                    break;
                },
                _ => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::InvalidPacketKind, 
                        Some(format!("Unexpected PacketKind, expected MetaData or MetaDataEnd, arrived:\n {:?}", packet.kind()))));
                }               
            }
        }
        let mut metadata = MetaData::from_bytes(metadata)?;
        if let Some(_) = metadata.file_name() {
            metadata.set_file_name(Some(location.unwrap().to_string_lossy().to_string()))
        };

        Ok(metadata)
    }
}

impl MetaData {
    
    /// Creates new [MetaData] from given data.
    ///
    /// # Arguments
    ///
    /// * `content` -- content of the [Message] so `message_length` can be set.
    /// * `message_kind`
    /// * `author` -- an [User] from whom is this [Message] coming.
    /// * `recipient_id` -- if sent by client that is [SERVER_ID](crate::config::SERVER_ID), otherwise id of the recipient.
    /// * `recipients` -- [Vec] of usernames of recipients.
    /// * `file_name` -- can be [Path](std::path::Path) or only name of file, depends on context.
    ///
    /// # Errors
    /// This method should not return an error.
    pub fn new(content: &Bytes,
           message_kind: MessageKind, author: UserLite,
           recipient_id: u32, recipients: Vec<String>,
           file_name: Option<String>) -> Result<MetaData, NetCommsError> {

        // Temporary metadata, to get length of metadata in number of packets.                  
        let temp_metadata = Self {
            message_kind,
            message_length: 0,
            datetime: Self::current_datetime().into_bytes(),
            author_id: author.id(),
            author_username: author.username(),
            recipient_id,
            recipients,
            file_name,
        };

        let metadata = temp_metadata.with_content_length(content.len());

        Ok(metadata)
    }

    pub fn with_content_length(self, content_length: usize) -> MetaData {

        let number_of_content_packets = Packet::number_of_packets(content_length);

        // This operation is not computationally heavy as there is only transfer of ownership, no cloning.
        let metadata_buff = self.into_buff();
        let n_of_metadata_packets = Packet::number_of_packets(metadata_buff.len());
                    
        let n_of_packets = n_of_metadata_packets + number_of_content_packets + 1;

        let mut metadata = MetaData::from_buff(&metadata_buff).unwrap();
        metadata.set_message_length(n_of_packets);

        metadata
    }

    /// Creates a new empty [MetaData].
    ///
    /// Datetime inside is correct.
    pub fn new_empty() -> Result<MetaData, NetCommsError> {

        let datetime = Self::current_datetime().into_bytes();

        Ok(MetaData {
            message_kind: MessageKind::Empty,
            message_length: 0,
            datetime,
            author_id: 0,
            author_username: "".to_string(),
            recipient_id: 0,
            recipients: vec![],
            file_name: None,
        })
    }

    pub fn from_data(message_kind: MessageKind,
                     message_length: u32,
                     datetime: Bytes,
                     author_id: u32,
                     author_username: String,
                     recipient_id: u32,
                     recipients: Vec<String>,
                     file_name: Option<String>) -> Self {

        MetaData { 
            message_kind,
            message_length,
            datetime,
            author_id,
            author_username,
            recipient_id,
            recipients,
            file_name,
        }
    }

    pub fn get_message_location(&self, location: &Path) -> PathBuf {

        let mut path = PathBuf::from(location);

        match self.message_kind {
            MessageKind::Request => path.push("requests"),
            MessageKind::Text | MessageKind::File => path.push("user_to_user"),
            MessageKind::SeverReply => path.push("server_replies"),
            _ => path.push("other"),
        }
        path.push(self.datetime_as_string());

        path
    }

    /// Returns [MessageKind] .
    pub fn message_kind(&self) -> MessageKind {
        self.message_kind.clone()
    }

    /// Returns `message_length` as number of [packets](super::Packet).
    pub fn message_length(&self) -> u32 {
        self.message_length 
    }

    /// Returns [DateTime<Utc>].
    /// 
    /// # Errors
    ///
    /// * This should not usually fail as it should be creating [DateTime<Utc>] from valid buffer,
    /// but will return an error if it from some reason fails to create [DateTime<Utc>] from buffer.
    pub fn datetime(&self) -> Result<DateTime<Utc>, NetCommsError> {
        Ok(DateTime::from_bytes(self.datetime.clone())?)
    }

    /// Returns a [DateTime<Utc>] as [String] in format: `year-month-day-hour-minute-second`.
    pub fn datetime_as_string(&self) -> String {
        
        let mut datetime_str = String::new();
        // This should not fail, so unwrap can be used.
        let datetime = self.datetime().unwrap().naive_utc();
        let date = datetime.date().to_string();
        let time = datetime.time().to_string();
        datetime_str.push_str(&date);
        datetime_str.push('-');
        datetime_str.push_str(&time);
        let datetime_str = datetime_str.replace(":", "-");

        datetime_str
    }

    /// Returns an `author_username`.
    pub fn author_username(&self) -> String {
        self.author_username.clone()
    }

    /// Returns an `author_id`.
    pub fn author_id(&self) -> u32 {
        self.author_id
    }

    /// Return a `recipient_id`, if [Message] was sent by client, this returns only [SERVER_ID](crate::config::SERVER_ID).
    pub fn recipient_id(&self) -> u32 {
        self.recipient_id
    }

    /// Returns `recipients`.
    pub fn recipients(&self) -> Vec<String> {
        self.recipients.clone()
    }

    /// Returns a `file_name`.
    pub fn file_name(&self) -> Option<String> {
        self.file_name.clone()
    }

    /// Sets `message_length`.
    pub fn set_message_length(&mut self, length: u32) {
        self.message_length = length;
    }    

    /// Sets [Message] `author_id`.
    pub fn set_author_id(&mut self, id: u32) {
        self.author_id = id;
    }

    /// Sets `recipient_id`.
    pub fn set_recipient_id(&mut self, recipient_id: u32) {
        self.recipient_id = recipient_id;
    }

    /// Sets `recipients`.
    pub fn set_recipients(&mut self, recipients: Vec<String>) {
        self.recipients = recipients;
    }

    /// Sets `file_name`.
    pub fn set_file_name(&mut self, name: Option<String>) {
        self.file_name = name;
    }

    /// Internal method used in [MetaData::new] and [MetaData::new_empty] to get current [[DateTime<Utc>]].
    fn current_datetime() -> DateTime<Utc> {
    
        let now = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();

        let naive_datetime = NaiveDateTime::from_timestamp(now as i64, 0);
    
        let time = DateTime::from_utc(naive_datetime, Utc); 
    
        time
    }
}
