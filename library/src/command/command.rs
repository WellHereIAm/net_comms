use crate::message::MessageKind;
use crate::user::UserUnchecked;

#[derive(Debug)]
pub enum Command {
    Register(UserUnchecked),
    Login(UserUnchecked),
    Yes,
    No,
    Send(MessageKind, usize, Vec<String>, Vec<u8>, Option<String>),    // Send commands have also info about message kind, recipients and content
    Unknown
}


// Move later!

#[derive(Debug)]
pub struct CommandError;

impl std::fmt::Display for CommandError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandError - TODO")
    }
}

impl std::error::Error for CommandError {}