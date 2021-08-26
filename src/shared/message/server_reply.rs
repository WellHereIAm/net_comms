use serde::{Serialize, Deserialize};
use ron::ser;
use ron::de;

use crate::error::{NetCommsError, NetCommsErrorKind};

/// Enum of all possible replies from server to client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerReply {
    /// Used when returning an error, [String] inside holds an error message.
    Error(String), // Later this string should be changed to use some kind of error enum, so client can recover from it.
    /// Used when there was a successful [Request::Register](crate::request::Request::Register) or [Request::Login](crate::request::Request::Login).
    User(User),
}
impl ServerReply {

    /// Transform self to [RON](ron) format
    ///
    /// # Errors 
    /// 
    /// * Will return an [NetCommsError] with kind [NetCommsErrorKind::SerializingFailed] if [serde] fails to serialize [ServerReply].
    pub fn to_ron(&self) -> Result<String, NetCommsError>{
        match ser::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::SerializingFailed,
                Some("Serializing ServerReplyKind struct failed.".to_string())))
        }
    }

    /// Creates MetaData from [RON](ron) if passed string is valid [ServerReply].
    ///
    /// # Errors 
    /// 
    /// * Will return an [NetCommsError] with kind [NetCommsErrorKind::DeserializingFailed] if [serde] fails to deserialize [ServerReply].
    pub fn from_ron(ron: &String) -> Result<Self, NetCommsError> {
        match de::from_str(ron) {
            Ok(metadata) => Ok(metadata),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::DeserializingFailed,
                Some("Deserializing of given RON to ServerReplyKind struct failed.".to_string())))
        }
    }
}