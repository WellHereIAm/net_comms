use library::{bytes::{Bytes, IntoBytes}, error::{NetCommsError, NetCommsErrorKind}, prelude::{IntoMessage, IntoRon, Packet, PacketKind}};
use shared::{Content, ImplementedMessage, MessageKind, MetaData, Request, config::{SERVER_ID, SERVER_USERNAME}, user::{User, UserUnchecked}};


# [derive(Debug)]
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

impl IntoMessage<'_, MetaData, Content> for Command {
    
    fn into_message(self) -> Result<ImplementedMessage, library::prelude::NetCommsError> {
        match self {
            Command::Send(message_kind, author, recipients, content, file_name) => {
                return from_send(message_kind, author, recipients, content.into_bytes(), file_name);    
            }
            Command::Register(user_unchecked, author) => {
                return from_register(user_unchecked, author);
            }
            Command::Login(user_unchecked, author) => {
                return from_login(user_unchecked, author);
            }
            _ => {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::UnknownCommand,
                    Some("Message::from_command() failed to create a message from given command.".to_string())));
            }
        }   
    }
}

/// Creates a [Message] from [Command::Send].
fn from_send(message_kind: MessageKind,
             author: User, recipients: Vec<String>,
             content: Bytes, file_name: Option<String>) -> Result<ImplementedMessage, NetCommsError> {

    let mut message = ImplementedMessage::new();

    let metadata = MetaData::new(&content, message_kind, author, SERVER_ID, recipients, file_name)?;
    message.set_metadata(metadata);

    let content = Content::with_data(content.to_string());
    message.set_content(content);

    let end_data = Packet::new(PacketKind::End, Bytes::new());
    message.set_end_data(end_data);

    Ok(message)    
}

/// Creates a [Message] from [Command::Register]. Used inside [Message::from_command].
fn from_register(user_unchecked: UserUnchecked, author: User) -> Result<ImplementedMessage, NetCommsError> {

    let mut message = ImplementedMessage::new();

    let request = Request::Register(user_unchecked);
    let content = IntoBytes::into_bytes(request.into_ron()?);

    // Recipient of Request will always be a server.
    let message_kind = MessageKind::Request;
    let recipients = vec![SERVER_USERNAME.to_string().clone()];
    let file_name = None;

    let metadata = MetaData::new(&content, message_kind, author, SERVER_ID, recipients, file_name)?;
    message.set_metadata(metadata);

    let content = Content::with_data(content.to_string());
    message.set_content(content);

    let end_data = Packet::new(PacketKind::End, Bytes::new());
    message.set_end_data(end_data);

    Ok(message)  
}

/// Creates a [Message] from [Command::Login]. Used inside [Message::from_command].
fn from_login(user_unchecked: UserUnchecked, author: User) -> Result<ImplementedMessage, NetCommsError> {

    let mut message = ImplementedMessage::new();

    let request = Request::Login(user_unchecked);
    let content = IntoBytes::into_bytes(request.into_ron()?);

    // Recipient of Request will always be a server.
    let message_kind = MessageKind::Request;
    let recipients = vec![SERVER_USERNAME.to_string().clone()];
    let file_name = None;

    let metadata = MetaData::new(&content, message_kind, author, SERVER_ID, recipients, file_name)?;
    message.set_metadata(metadata);

    let content = Content::with_data(content.to_string());
    message.set_content(content);

    let end_data = Packet::new(PacketKind::End, Bytes::new());
    message.set_end_data(end_data);

    Ok(message)  
}