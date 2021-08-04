use std::net::TcpStream;

use crate::prelude::{ADDR, PORT, UserUnchecked};

#[derive(Debug)]
pub struct RequestErr {}

impl std::fmt::Display for RequestErr {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Some error, implement later.")
    }
}

impl std::error::Error for RequestErr {}

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
    // Later actually establish connection with the server.
    pub fn get_id() -> Result<[u8; 4], RequestErr> {

        return Ok([1, 1, 1, 1]);

        let socket = format!("{}:{}", ADDR, PORT);
        
        let stream = match TcpStream::connect(socket) {
            Ok(stream) => stream,
            Err(_) => todo!(),
        };
    }
}