use ron::de;
use serde::Deserialize;

use crate::error::{NetCommsError, NetCommsErrorKind};

/// Trait with default method, that allows to create an implementor from given [RON](ron).
pub trait FromRon<'a> 
where
    Self: Sized + Deserialize<'a> {

    /// Creates an implementor from [RON](ron).
    ///
    /// # Errors
    ///
    /// Will return [NetCommsError] with kind [NetCommsErrorKind::DeserializingFailed]
    /// if it fails to deserialize given string to implementor.
    fn from_ron(ron: &'a String) -> Result<Self, NetCommsError> {
        match de::from_str(ron) {
            Ok(metadata) => Ok(metadata),
            Err(_) => Err(NetCommsError::new(
                NetCommsErrorKind::DeserializingFailed,
                Some("Deserializing of given RON to struct failed.".to_string())))
        }
    }
}