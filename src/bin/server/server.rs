use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::net::Ipv4Addr;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};
use ron::ser;
use ron::de;

use library::error::{NetCommsError, NetCommsErrorKind};
use library::message::Message;
use library::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub ip: String,
    pub port_start: usize,
    pub port_end: usize,
    pub save_location: PathBuf,
}

impl ServerSettings {

    pub fn new(location: &Path) -> Result<Self, NetCommsError> {

        match fs::File::open(location) {
            Ok(mut file) => {
                let mut buffer = String::new();
                if let Err(_) = file.read_to_string(&mut buffer) {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::ReadingFromFileFailed,
                        None));
                } 

                match de::from_str(&buffer) {
                    Ok(server_settings) => return Ok(server_settings),
                    Err(_) => Err(NetCommsError::new(
                        NetCommsErrorKind::DeserializingFailed,
                        Some("Deserializing of given RON to ServerSettings struct failed.".to_string()))),
                }

            },
            Err(_) => return Err(NetCommsError::new(
                NetCommsErrorKind::OpeningFileFailed,
                None)),
        }
    }    
}



pub struct Server {
   ip: Ipv4Addr,
   port: RangeInclusive<usize>,
   save_location: PathBuf,
   waiting_messages: Arc<Mutex<HashMap<usize, Vec<Message>>>>,
   users: Arc<Mutex<HashMap<String, User>>>,
   ids: Arc<Mutex<usize>>,
}

impl Server {
    
    pub fn new(location: &Path) -> Result<Self, NetCommsError> {
        let server_settings = ServerSettings::new(location)?;
        //let ip = server_settings.ip.split(".").map(|x|)
        todo!()
    }
}