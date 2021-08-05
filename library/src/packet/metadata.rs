use std::time::SystemTime;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use ron::ser;
use ron::ser::PrettyConfig;
use ron::de;

use crate::buffer::{ToBuffer, FromBuffer};
use crate::message::MessageKind;


/// This struct holds metadata of each message to be sent or received.
// CAN AND MOST LIKELY WILL GROW.
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
    /// and first encodes to RON format which is encoded to buffer.
    fn to_buff(self) -> Vec<u8> {
        self.to_ron().to_buff()
    }    
}

impl FromBuffer for MetaData {

    /// Wrapper around MetaData::from_ron(), which takes String to return MetaData.
    /// Takes an ownership of buff and returns MetaData.
    fn from_buff(buff: Vec<u8>) -> Self {
        MetaData::from_ron(&String::from_buff(buff))
    }
}

impl MetaData {
    
    /// Creates a new MetaData from given arguments.
    pub fn new(message_kind: MessageKind, message_length: usize,
               author_id: usize,
               recipient_id: usize, recipients: Vec<String>,
               file_name: Option<String>) -> Self {

        let datetime = Self::datetime().to_buff();
        
        MetaData {
            message_kind,
            message_length,
            datetime,
            author_id,
            recipient_id,
            recipients,
            file_name,
        }
    }

    /// Creates a new empty MetaData. Datetime inside is correct.
    pub fn new_empty() -> Self {

        let datetime = Self::datetime().to_buff();

        MetaData {
            message_kind: MessageKind::Empty,
            message_length: 0,
            datetime,
            author_id: 0,
            recipient_id: 0,
            recipients: vec![],
            file_name: None,
        }
    }

    /// Method returns a RON from MetaData.
    pub fn to_ron(&self) -> String {
        ser::to_string(&self).unwrap() // CAN I USE UNWRAP?
    }

    /// Method returns a pretty formatted RON from MetaData.
    /// Optional config gives a config to use for formatting.
    pub fn to_ron_pretty(&self, config: Option<PrettyConfig>) -> String {

        let config = match config {
            Some(config) => config,
            None => {
                let config = PrettyConfig::new()
                    .with_depth_limit(4)
                    .with_indentor("\t".to_owned())
                    .with_decimal_floats(true);
                config
            },
        };

        ser::to_string_pretty(&self, config).unwrap() // CAN I USE UNWRAP?

    }

    /// Returns MetaData from RON.
    pub fn from_ron(ron: &String) -> Self {
        de::from_str(ron).unwrap() // CAN I USE UNWRAP?
    }

    pub fn message_kind(&self) -> MessageKind {
        self.message_kind.clone()
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
