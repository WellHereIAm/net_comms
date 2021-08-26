use serde::{Serialize, Deserialize};

use library::ron::{IntoRon, FromRon};
use library::user::UserUnchecked;
use library::message::{Message, MessageKind, IntoMessage};
use library::packet::{MetaData, Packet, PacketKind};
use library::error::NetCommsError;
use library::user::User;
use library::buffer::IntoBuffer;
use library::config::{SERVER_USERNAME, SERVER_ID};


/// Holds data about requests from client to server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    /// Request to login with [UserUnchecked] inside.
    Login(UserUnchecked),

    /// Request to register with [UserUnchecked] inside.
    Register(UserUnchecked),

    /// Request to get any [messages](crate::message::Message) that were sent to requesting client.
    GetWaitingMessagesAuto,

    /// Used if some method fails to recognize the [Request].
    Unknown,    
}

impl IntoRon for Request {}
impl FromRon<'_> for Request {}

pub enum RequestRaw {
    /// Request to login with [UserUnchecked] inside.
    Login(UserUnchecked, User),

    /// Request to register with [UserUnchecked] inside.
    Register(UserUnchecked, User),

    /// Request to get any [messages](crate::message::Message) that were sent to requesting client.
    GetWaitingMessagesAuto(User),

    /// Used if some method fails to recognize the [Request].
    Unknown(User),    
}

impl IntoMessage for RequestRaw {
    
    fn into_message(self) -> Result<Message, NetCommsError> {

        let (request, author) = match self {
            RequestRaw::Login(user_unchecked, author) => (Request::Login(user_unchecked), author),
            RequestRaw::Register(user_unchecked, author) => (Request::Register(user_unchecked), author),
            RequestRaw::GetWaitingMessagesAuto(author) => (Request::GetWaitingMessagesAuto, author),
            RequestRaw::Unknown(author) => (Request::Unknown, author),
        };

        let mut message = Message::new()?;
        let content = request.into_ron()?.into_buff()?;

        // Recipient of Request will always be a server.
        let message_kind = MessageKind::Request;
        let recipients = vec![SERVER_USERNAME.to_string().clone()];
        let file_name = None;

        let metadata = MetaData::new(&content, message_kind, author, SERVER_ID, recipients, file_name)?;
        message.set_metadata(metadata);

        message.set_content(content);

        let end_data = Packet::new(PacketKind::End, Vec::new());
        message.set_end_data(end_data);

        Ok(message)  
    }
}