use serde::{Serialize, Deserialize};

use crate::buffer::FromBuffer;
use crate::message::{MessageKind, Message};
use crate::pretty_structs::{MetaDataPretty, PacketPretty};
use crate::ron::{ToRon, FromRon};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePretty {
    kind: MessageKind,
    metadata: MetaDataPretty,
    content: String,
    end_data: PacketPretty,
}

impl ToRon for MessagePretty {}
impl FromRon<'_> for MessagePretty {}

impl MessagePretty {
    
    pub fn from_message(message: &Message) -> Self {
        let kind = message.kind();
        let metadata = MetaDataPretty::from_metadata(&message.metadata());
        let content = String::from_buff(message.clone().content_owned()).unwrap();
        let end_data = PacketPretty::from_packet(&message.end_data());

        MessagePretty {
            kind,
            metadata,
            content,
            end_data,
        }
    } 
}