
use crate::error::NetCommsErrorKind;

#[derive(Debug)]
pub struct NetCommsError {
    kind: NetCommsErrorKind,
}

impl std::fmt::Display for NetCommsError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }    
}

impl std::error::Error for NetCommsError {}

impl NetCommsError {

    pub fn new(kind: NetCommsErrorKind) -> Self {
        NetCommsError {kind}
    }
    
}