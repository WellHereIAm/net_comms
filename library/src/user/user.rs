use serde::{Serialize, Deserialize};
use ron::ser;
use ron::de;

use crate::error::{NetCommsError, NetCommsErrorKind};
use crate::config::{UNKNOWN_USER_ID, UNKNOWN_USERNAME};


/// Holds data about user.
///
/// # Fields
///
/// * `id` -- assigned by server.
/// * `username`
/// * `password`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: u32,
    username: String,
    password: String, // This of course needs to be hashed.
}

impl Default for User {

    /// Creates a default [User] with [UNKNOWN_USER_ID] and [UNKNOWN_USERNAME].
    fn default() -> Self {
        User {
            id: UNKNOWN_USER_ID,
            username: UNKNOWN_USERNAME.to_string(),
            password: "None".to_string(),
        }
    }
}

impl User {

    /// Creates a new [User].    
    pub fn new(id: u32, username: String, password: String) -> Self {
        User {
            id,
            username,
            password,
        }
    }

    /// Returns a [RON](ron) from [User].
    ///
    /// # Errors
    /// * Will return [NetCommsError] with kind [NetCommsErrorKind::SerializingFailed] if it fails to serialize this [User].
    pub fn to_ron(&self) -> Result<String, NetCommsError>{
        match ser::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::SerializingFailed,
                Some("Serializing User struct failed.".to_string())))
        }
    }

    /// Creates a [User] from [RON](ron).
    ///
    /// # Errors
    ///
    /// Will return [NetCommsError] with kind [NetCommsErrorKind::DeserializingFailed]
    /// if it fails to deserialize given string to [User].
    pub fn from_ron(ron: &String) -> Result<Self, NetCommsError> {
        dbg!(&ron);
        match de::from_str(ron) {
            Ok(metadata) => Ok(metadata),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::DeserializingFailed,
                Some("Deserializing of given RON to User struct failed.".to_string())))
        }
    }
    
    /// Returns `users_id`.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns `username`.
    pub fn username(&self) -> String {
        self.username.clone()
    }

    /// Returns `password`.
    // Not to sure about this one.
    pub fn password(&self) -> String {
        self.password.clone()
    }
}