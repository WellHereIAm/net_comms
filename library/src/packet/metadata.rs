use std::time::SystemTime;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::buffer::{ToBuffer, FromBuffer};
use crate::error::NetCommsError;
use crate::message::{Message, MessageKind};
use crate::ron::{ToRon, FromRon};
use crate::user::User;


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
    datetime: Vec<u8>,  
    author_id: u32,   
    author_username: String,
    recipient_id: u32, 
    recipients: Vec<String>,
    file_name: Option<String>,  
}

impl ToRon for MetaData {}
impl FromRon<'_> for MetaData {}

impl ToBuffer for MetaData {

    /// This takes an ownership of self and first encodes MetaData to RON format which is then encoded to buffer.
    fn to_buff(self) -> Result<Vec<u8>, NetCommsError> {
        self.to_ron()?.to_buff()
    }    
}

impl FromBuffer for MetaData {

    /// Wrapper around MetaData::from_ron(), which takes String to return MetaData.
    ///
    /// Takes an ownership of buff and returns MetaData.
    /// This implementation does not check if the buffer has correct length as it can vary.
    fn from_buff(buff: Vec<u8>) -> Result<MetaData, NetCommsError> {
        MetaData::from_ron(&String::from_buff(buff)?)
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
    pub fn new(content: &Vec<u8>,
           message_kind: MessageKind, author: User,
           recipient_id: u32, recipients: Vec<String>,
           file_name: Option<String>) -> Result<MetaData, NetCommsError> {

        // Temporary metadata, to get length of metadata in number of packets.                  
        let temp_metadata = Self {
            message_kind,
            message_length: 0,
            datetime: Self::current_datetime().to_buff()?,
            author_id: author.id(),
            author_username: author.username(),
            recipient_id,
            recipients,
            file_name,
        };

        let temp_metadata_buffer = temp_metadata.to_buff()?;    
        let n_of_metadata_packets = Message::number_of_packets(&temp_metadata_buffer);        
        let n_of_content_packets = Message::number_of_packets(content);

        // Adds number of MetaData packets to number of Content packets to one End packet.
        let message_length = n_of_metadata_packets + n_of_content_packets + 1; 

        let mut metadata = MetaData::from_buff(temp_metadata_buffer)?;
        metadata.set_message_length(message_length);

        Ok(metadata)
}

    /// Creates a new empty [MetaData].
    ///
    /// Datetime inside is correct.
    pub fn new_empty() -> Result<MetaData, NetCommsError> {

        let datetime = Self::current_datetime().to_buff()?;

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
        Ok(DateTime::from_buff(self.datetime.clone())?)
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
