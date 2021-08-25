use crate::error::NetCommsError;
use crate::message::Message;

pub trait ToMessage {

    fn to_message(self) -> Result<Message, NetCommsError>;
}