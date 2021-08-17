use std::time::SystemTime;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use ron::ser::{self, PrettyConfig};
use ron::de;

use crate::buffer::{ToBuffer, FromBuffer};
use crate::message::MessageKind;
use crate::error::{NetCommsError, NetCommsErrorKind};
use crate::prelude::{Message, User};


/// This struct holds metadata of each message to be sent or received.
// Can grow in future.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    message_kind: MessageKind,
    /// Length of message in number of packets.
    message_length: usize,  
    /// Encoded chrono::Datetime<Utc> to Vec<u8> to ease serde serializing and deserializing.
    datetime: Vec<u8>,  
    /// If created by server, author_id is SERVER_ID, so id of the author will not be sent to recipient.
    author_id: usize,   
    author_name: String,
    /// If created by client, recipient id is SERVER_ID, not ids of recipients.
    recipient_id: usize, 
    recipients: Vec<String>,
    /// If Some, String holds a file name and file extension.
    file_name: Option<String>,  
}

impl ToBuffer for MetaData {

    /// This takes an ownership of self
    /// and first encodes MetaData to RON format which is then encoded to buffer.
    fn to_buff(self) -> Result<Vec<u8>, NetCommsError> {
        self.to_ron()?.to_buff()
    }    
}

impl FromBuffer for MetaData {

    /// Wrapper around MetaData::from_ron(), which takes String to return MetaData.
    /// Takes an ownership of buff and returns MetaData.
    /// This implementation does not check if the buffer has correct length as it can vary.
    fn from_buff(buff: Vec<u8>) -> Result<MetaData, NetCommsError> {
        MetaData::from_ron(&String::from_buff(buff)?)
    }
}

impl MetaData {
    
    /// Creates new MetaData from given data.
    pub fn new(content: &Vec<u8>,
           message_kind: MessageKind, author: User,
           recipient_id: usize, recipients: Vec<String>,
           file_name: Option<String>) -> Result<MetaData, NetCommsError> {

        // Temporary metadata, to get length of metadata in number of packets.                  
        let temp_metadata = Self {
            message_kind,
            message_length: 0,
            datetime: Self::current_datetime().to_buff()?,
            author_id: author.id(),
            author_name: author.username(),
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

    /// Creates a new empty MetaData. Datetime inside is correct.
    pub fn new_empty() -> Result<MetaData, NetCommsError> {

        let datetime = Self::current_datetime().to_buff()?;

        Ok(MetaData {
            message_kind: MessageKind::Empty,
            message_length: 0,
            datetime,
            author_id: 0,
            author_name: "".to_string(),
            recipient_id: 0,
            recipients: vec![],
            file_name: None,
        })
    }

    /// Method returns a RON from MetaData if serializing is successful.
    pub fn to_ron(&self) -> Result<String, NetCommsError>{
        match ser::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::SerializingFailed,
                Some("Serializing MetaData struct failed.".to_string())))
        }
    }

    /// Method returns a pretty formatted RON from MetaData if serializing is successful.
    /// Optional config gives a config to use for formatting.
    pub fn to_ron_pretty(&self, config: Option<PrettyConfig>) -> Result<String, NetCommsError> {

        let config = match config {
            Some(config) => config,
            None => {
                let config = PrettyConfig::new()
                    .with_depth_limit(4)
                    .with_indentor("\n\r".to_owned())
                    .with_decimal_floats(true);
                config
            },
        };

       match ser::to_string_pretty(&self, config) {
            Ok(serialized) => Ok(serialized),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::SerializingFailed,
                Some("Serializing MetaData struct failed.".to_string())))
        }

    }

    /// Creates MetaData from RON if passed string is valid.
    pub fn from_ron(ron: &String) -> Result<Self, NetCommsError> {
        match de::from_str(ron) {
            Ok(metadata) => Ok(metadata),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::DeserializingFailed,
                Some("Deserializing of given RON to MetaData struct failed.".to_string())))
        }
    }

    /// Returns MessageKind of MetaData.
    pub fn message_kind(&self) -> MessageKind {
        self.message_kind.clone()
    }

    /// Returns message_length as number of packet to be send or received.
    pub fn message_length(&self) -> usize {
        self.message_length 
    }

    pub fn datetime(&self) -> Result<DateTime<Utc>, NetCommsError> {
        Ok(DateTime::from_buff(self.datetime.clone())?)
    }

    /// Returns an author username.
    pub fn author_name(&self) -> String {
        self.author_name.clone()
    }

    /// Returns an author id.
    pub fn author_id(&self) -> usize {
        self.author_id.clone()
    }

    /// Return a recipient id, if Message was sent by client, this returns only 'SERVER_ID'.
    pub fn recipient_id(&mut self) -> usize {
        self.recipient_id
    }

    /// Returns recipients.
    pub fn recipients(&self) -> Vec<String> {
        self.recipients.clone()
    }

    /// Returns file_name if it exists, otherwise None.
    pub fn file_name(&self) -> Option<String> {
        self.file_name.clone()
    }

    /// Sets message_length to MetaData.
    pub fn set_message_length(&mut self, length: usize) {
        self.message_length = length;
    }    

    /// Sets file_name to MetaData.
    pub fn set_file_name(&mut self, name: Option<String>) {
        self.file_name = name;
    }

    /// Internal method used in MetaData::new() and MetaData::new_empty() to get current datetime.
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
