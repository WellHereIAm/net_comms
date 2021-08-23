use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};
use ron::ser;
use ron::de;

use library::message::Message;
use library::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    ip: String,
    port_start: usize,
    port_end: usize,
    save_location: PathBuf,
}

pub struct Server {
   ip: Ipv4Addr,
   port: RangeInclusive<usize>,
   save_location: PathBuf,
   waiting_messages: Arc<Mutex<HashMap<usize, Vec<Message>>>>,
   users: Arc<Mutex<HashMap<String, User>>>,
   ids: Arc<Mutex<usize>>,
}