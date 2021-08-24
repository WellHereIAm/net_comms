use std::path::{Path, PathBuf};
use std::fs;
use std::io::Read;

use serde::{Serialize, Deserialize};
use ron::de;

use library::error::{NetCommsError, NetCommsErrorKind};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    save_location: PathBuf,
}

impl ClientConfig {

    pub fn new(config_location: &Path) -> Result<Self, NetCommsError> {

        match fs::File::open(config_location) {
            Ok(mut file) => {
                let mut buffer = String::new();
                if let Err(_) = file.read_to_string(&mut buffer) {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::ReadingFromFileFailed,
                        None));
                } 

                match de::from_str(&buffer) {
                    Ok(server_settings) => return Ok(server_settings),
                    Err(e) => Err(NetCommsError::new(
                        NetCommsErrorKind::DeserializingFailed,
                        Some(format!("Deserializing of given RON to ServerSettings struct failed.\n{:?}", e)))),
                }

            },
            Err(_) => return Err(NetCommsError::new(
                NetCommsErrorKind::OpeningFileFailed,
                None)),
        }
    }    
}