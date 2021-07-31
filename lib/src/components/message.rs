use chrono::{DateTime, NaiveDateTime, Utc};

use crate::Packet;


pub struct Message {
    author: [u8; 4],
    recipient: [u8; 4], // Sender sends server id, server will match recipient name in metadata packet  
    time: DateTime<Utc>,
    content: Vec<Packet>
}
