use crate::{packet::MetaData, prelude::{Message, NetCommsError, UserUnchecked}};

pub enum Request {
    GetWaitingMessages,
    Unknown,    
}

impl Request {
    
    pub fn to_message(&self) -> Result<Message, NetCommsError> {

        let mut message = Message::new()?;
        Ok(message)
    }
}

#[derive(Debug)]
pub enum RequestKind {

    GetId(Box<RequestKind>),
    Register(UserUnchecked),
    Login(UserUnchecked),
    Unknown,
}

impl std::fmt::Display for RequestKind {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestKind::GetId(rqst_kind) => {
                match **rqst_kind {
                    RequestKind::Register(_) => write!(f, "GetId(Register)"),
                    RequestKind::Login(_) => write!(f, "GetId(Login)"),
                    _ => write!(f, "GetId(Wrong request)")
                }
            },
            RequestKind::Register(_) => write!(f, "Register"),
            RequestKind::Login(_) => write!(f, "Login"),
            RequestKind::Unknown => write!(f, "Unknown"),
        }
    }    
}

impl RequestKind {
}