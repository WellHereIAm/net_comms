use crate::message::MessageKind;
use crate::prelude::User;
use crate::user::UserUnchecked;

/// Stores command in a way that enables simple sending using RON format and [Message](crate::message::Message) through [TcpStream](std::net::TcpStream).
/// Is usually constructed by [CommandRaw::process](crate::command::CommandRaw::process).
#[derive(Debug)]
pub enum Command {
    /// Command containing [UserUnchecked] with username and password that is user attempting to use to register.
    /// [User] is usually a default user.
    Register(UserUnchecked, User),

    /// Command containing [UserUnchecked] with username and password that is user attempting to use to login.
    /// [User] is usually a default user.
    Login(UserUnchecked, User),

    /// Command containing the [User] that used this command.
    Yes(User),

    /// Command containing the [User] that used this command.
    No(User),

    /// Command that is containing all necessary information to construct a [Message](crate::message::Message). 
    /// * [MessageKind]
    /// * [User] -- author that used this command.
    /// * [Vec<String>] -- recipients of this [Message](crate::message::Message).
    /// * [Vec<u8>] -- content of this [Message](crate::message::Message).
    /// * [Option<String>] -- information if content of this [Message](crate::message::Message) is a file.
    Send(MessageKind, User, Vec<String>, Vec<u8>, Option<String>), 

    /// Used to signalize that created command is an unknown command.
    Unknown
}
