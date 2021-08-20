use serde::{Serialize, Deserialize};
use ron::ser;
use ron::de;

use crate::config::UNKNOWN_USER_ID;
use crate::prelude::{NetCommsError, NetCommsErrorKind};

/// Struct to hold data about user, most likely will grow in future by a lot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: usize,
    username: String,
    password: String, // This of course needs to be hashed.
}

impl Default for User {

    fn default() -> Self {
        User {
            id: 1,
            username: UNKNOWN_USER_ID.to_string(),
            password: "None".to_string(),
        }
    }
}

impl User {

    /// Creates new User.    
    pub fn new(id: usize, username: String, password: String) -> Self {
        User {
            id,
            username,
            password,
        }
    }

    pub fn to_ron(&self) -> Result<String, NetCommsError>{
        match ser::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::SerializingFailed,
                Some("Serializing User struct failed.".to_string())))
        }
    }

    /// Creates MetaData from RON if passed string is valid.
    pub fn from_ron(ron: &String) -> Result<Self, NetCommsError> {
        dbg!(&ron);
        match de::from_str(ron) {
            Ok(metadata) => Ok(metadata),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::DeserializingFailed,
                Some("Deserializing of given RON to User struct failed.".to_string())))
        }
    }
    
    /// Returns users id.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns users username.
    pub fn username(&self) -> String {
        self.username.clone()
    }

    /// Returns users
    // Not to sure about this one.
    pub fn password(&self) -> String {
        self.password.clone()
    }

}