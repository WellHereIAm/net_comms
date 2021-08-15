use crate::{packet::MetaData, prelude::{Message, NetCommsError, UserUnchecked}};

pub enum Request {
    Login(UserUnchecked),
    Register(UserUnchecked),
    GetWaitingMessages,
    Unknown,    
}

impl Request {
    
    pub fn to_message(&self) -> Result<Message, NetCommsError> {

        let mut message = Message::new()?;
        Ok(message)
    }
}