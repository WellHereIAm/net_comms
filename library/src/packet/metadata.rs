use std::time::SystemTime;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use ron::ser::{self, PrettyConfig};
use ron::de;

use crate::buffer::{ToBuffer, FromBuffer};
use crate::message::MessageKind;
use crate::error::{NetCommsError, NetCommsErrorKind};


/// This struct holds metadata of each message to be sent or received.
// Can grow in future.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    message_kind: MessageKind,
    message_length: usize,  // Length of message in number of packets.
    datetime: Vec<u8>,  // Encoded chrono::Datetime<Utc> to Vec<u8> to ease serde serializing and deserializing.
    author_id: usize,
    recipient_id: usize, // In future maybe get rid of this field as now it´s just adding nothing.
    recipients: Vec<String>,
    file_name: Option<String>,  // If Some, String holds a file name and extension.
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
    fn from_buff(buff: Vec<u8>) -> Result<MetaData, NetCommsError> {
        MetaData::from_ron(&String::from_buff(buff)?)
    }
}

impl MetaData {
    
    /// Creates a new MetaData from given arguments.
    pub fn new(message_kind: MessageKind, message_length: usize,
               author_id: usize,
               recipient_id: usize, recipients: Vec<String>,
               file_name: Option<String>) ->  Result<MetaData, NetCommsError> {

        let datetime = Self::datetime().to_buff()?;
        
        Ok(MetaData {
            message_kind,
            message_length,
            datetime,
            author_id,
            recipient_id,
            recipients,
            file_name,
        })
    }

    /// Creates a new empty MetaData. Datetime inside is correct.
    pub fn new_empty() -> Result<MetaData, NetCommsError> {

        let datetime = Self::datetime().to_buff()?;

        Ok(MetaData {
            message_kind: MessageKind::Empty,
            message_length: 0,
            datetime,
            author_id: 0,
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

    /// Returns file_name if it exists, otherwise None.
    pub fn file_name(&self) -> Option<String> {
        self.file_name.clone()
    }

    /// Returns message_length as number of packet to be send or received.
    pub fn message_length(&self) -> usize {
        self.message_length 
    }

    /// Sets file_name to MetaData.
    pub fn set_file_name(&mut self, name: Option<String>) {
        self.file_name = name;
    }

    /// Sets message_length to MetaData.
    pub fn set_message_length(&mut self, length: usize) {
        self.message_length = length;
    }    

    /// Internal method used in MetaData::new() and MetaData::new_empty() to get current datetime.
    fn datetime() -> DateTime<Utc> {
    
        let now = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();

        let naive_datetime = NaiveDateTime::from_timestamp(now as i64, 0);
    
        let time = DateTime::from_utc(naive_datetime, Utc); 
    
        time
    }
}
