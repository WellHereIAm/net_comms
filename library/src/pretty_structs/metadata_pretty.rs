
use serde::{Serialize, Deserialize};

use crate::message::MessageKind;
use crate::ron::{ToRon, FromRon};
use crate::packet::MetaData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaDataPretty {
    message_kind: MessageKind,
    message_length: u32,  
    datetime: String,  
    author_id: u32,   
    author_username: String,
    recipient_id: u32, 
    recipients: Vec<String>,
    file_name: Option<String>,  
}

impl ToRon for MetaDataPretty {}
impl FromRon<'_> for MetaDataPretty {}

impl MetaDataPretty {
    
    pub fn from_metadata(metadata: &MetaData) -> Self {
        
        MetaDataPretty {
            message_kind: metadata.message_kind(),
            message_length: metadata.message_length(),
            datetime: metadata.datetime_as_string(),
            author_id: metadata.author_id(),
            author_username: metadata.author_username(),
            recipient_id: metadata.recipient_id(),
            recipients: metadata.recipients(),
            file_name: metadata.file_name(),
        }
    }
}