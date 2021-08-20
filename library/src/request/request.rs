use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use ron::ser::{self, PrettyConfig};
use ron::de;

use crate::prelude::NetCommsErrorKind;
use crate::{packet::MetaData, prelude::{Message, NetCommsError, UserUnchecked}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    Login(UserUnchecked),
    Register(UserUnchecked),
    GetWaitingMessages,
    Unknown,    
}

impl Request {

    pub fn to_ron(&self) -> Result<String, NetCommsError>{
        match ser::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::SerializingFailed,
                Some("Serializing Request struct failed.".to_string())))
        }
    }

    /// Creates MetaData from RON if passed string is valid.
    pub fn from_ron(ron: &String) -> Result<Self, NetCommsError> {
        match de::from_str(ron) {
            Ok(metadata) => Ok(metadata),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::DeserializingFailed,
                Some("Deserializing of given RON to Request struct failed.".to_string())))
        }
    }
    
    pub fn to_message(&self) -> Result<Message, NetCommsError> {

        let mut message = Message::new()?;
        Ok(message)
    }
}