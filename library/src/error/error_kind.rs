#[derive(Debug)]
pub enum NetCommsErrorKind {
    WrongCommand,    
}

impl std::fmt::Display for NetCommsErrorKind {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetCommsErrorKind::WrongCommand => write!(f, "{}", "Wrong Command."),
        }
    }
    
}