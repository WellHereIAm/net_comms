use crate::prelude::NetCommsError;
use crate::prelude::NetCommsErrorKind;
use serde::{Serialize, Deserialize};
use ron::ser;
use ron::de;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerReplyKind {
    Error(String), // Message
    User,
}
impl ServerReplyKind {

    pub fn to_ron(&self) -> Result<String, NetCommsError>{
        match ser::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::SerializingFailed,
                Some("Serializing ServerReplyKind struct failed.".to_string())))
        }
    }

    /// Creates MetaData from RON if passed string is valid.
    pub fn from_ron(ron: &String) -> Result<Self, NetCommsError> {
        match de::from_str(ron) {
            Ok(metadata) => Ok(metadata),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::DeserializingFailed,
                Some("Deserializing of given RON to ServerReplyKind struct failed.".to_string())))
        }
    }
}