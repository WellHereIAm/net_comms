use std::time::SystemTime;

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::{Packet, settings};

pub enum MessageKind {
    Empty,
    Request,
    Text,
    File,
    Unknown,
} 

pub struct Message {
    kind: MessageKind,
    author: [u8; 4],
    recipient: [u8; 4], // Sender sends server id, server will match recipient name in metadata packet  
    time: DateTime<Utc>,
    content: Vec<Packet>
}

impl Message {

    pub fn new(kind: MessageKind, author: [u8; 4]) -> Self {

        let now = SystemTime::now()
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs() as i64;         
        let naive_datetime = NaiveDateTime::from_timestamp(now, 0);
        let time = DateTime::from_utc(naive_datetime, Utc);

        let metadata = Packet::new(kind: Meta)

        Message {
            kind,
            author,
            recipient: settings::SERVER_ID,
            time,
            content: Vec::new(),
        }
    }
}
