use serde::{Serialize, Deserialize};
use ron::ser;
use ron::de;

use crate::error::{NetCommsError, NetCommsErrorKind};
use crate::user::UserUnchecked;


/// Holds data about requests from client to server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    /// Request to login with [UserUnchecked] inside.
    Login(UserUnchecked),

    /// Request to register with [UserUnchecked] inside.
    Register(UserUnchecked),

    /// Request to get any [messages](crate::message::Message) that were sent to requesting client.
    GetWaitingMessages,

    /// Used if some method fails to recognize the [Request].
    Unknown,    
}

impl Request {

    /// Returns a [RON](ron) from [Request].
    ///
    /// # Errors
    /// * Will return [NetCommsError] with kind [NetCommsErrorKind::SerializingFailed] if it fails to serialize this [Request].
    pub fn to_ron(&self) -> Result<String, NetCommsError>{
        match ser::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::SerializingFailed,
                Some("Serializing Request struct failed.".to_string())))
        }
    }

    /// Creates [Request] from [RON](ron).
    ///
    /// # Errors
    ///
    /// Will return [NetCommsError] with kind [NetCommsErrorKind::DeserializingFailed]
    /// if it fails to deserialize given string to [Request].
    pub fn from_ron(ron: &String) -> Result<Self, NetCommsError> {
        match de::from_str(ron) {
            Ok(metadata) => Ok(metadata),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::DeserializingFailed,
                Some("Deserializing of given RON to Request struct failed.".to_string())))
        }
    }
}