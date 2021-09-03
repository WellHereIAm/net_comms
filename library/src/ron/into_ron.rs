use serde::Serialize;
use ron::ser::{self, PrettyConfig};

use crate::error::{NetCommsError, NetCommsErrorKind};

/// Trait with default methods that allow implementors parse them to [RON](ron) format.
pub trait IntoRon
where 
    Self: Serialize {
    
    /// Returns a [RON](ron) from implementor.
    ///
    /// # Errors
    /// * Will return [NetCommsError] with kind [NetCommsErrorKind::SerializingFailed] if it fails to serialize this implementor.
    fn into_ron(&self) -> Result<String, NetCommsError> {

        match ser::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(e) => Err(NetCommsError::new(
                NetCommsErrorKind::SerializingFailed,
                Some(format!("Serializing struct failed.\n{}", e))))
        }
    }

    /// Returns a `pretty` formatted [RON](ron) from implementor.
    ///
    /// Optional `config` gives a [PrettyConfig] to use for formatting, but there is default one set.
    ///
    /// # Errors
    /// * Will return [NetCommsError] with kind [NetCommsErrorKind::SerializingFailed] if it fails to serialize implementor.
    fn into_ron_pretty(&self, config: Option<PrettyConfig>) -> Result<String, NetCommsError> {
        
        let config = match config {
            Some(config) => config,
            None => {
                let config = PrettyConfig::new()
                    .with_depth_limit(4)
                    .with_decimal_floats(true);
                config
            },
        };

       match ser::to_string_pretty(&self, config) {
            Ok(serialized) => Ok(serialized),
            Err(e) => Err(NetCommsError::new(
                NetCommsErrorKind::SerializingFailed,
                Some(format!("Serializing struct failed.\n{}", e))))
        }

    }
}