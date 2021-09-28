use nardol::bytes::{Bytes, FromBytes, IntoBytes};
use serde::{Serialize, Deserialize};

use nardol::ron::{FromRon, ToRon};
use nardol::message::{Message, IntoMessage, MetaDataType, ContentType};
use nardol::error::NetCommsError;
use nardol::packet::{Packet, PacketKind};

use crate::config::{SERVER_ID, SERVER_USERNAME};
use crate::message::{MessageKind, MetaData, Content};
use crate::user::{User, UserLite, UserUnchecked};

use crate::ImplementedMessage;


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

impl ToRon for Request {}
impl FromRon<'_> for Request {}

pub enum RequestRaw {
    /// Request to login with [UserUnchecked] inside.
    Login(UserUnchecked, UserLite),

    /// Request to register with [UserUnchecked] inside.
    Register(UserUnchecked, UserLite),

    /// Request to get any [messages](crate::message::Message) that were sent to requesting client.
    GetWaitingMessagesAuto(UserLite),

    /// Used if some method fails to recognize the [Request].
    Unknown(UserLite),    
}

impl IntoMessage<'_, MetaData, Content> for RequestRaw {
    
    fn into_message(self) -> Result<ImplementedMessage, NetCommsError> {

        let (request, author) = match self {
            RequestRaw::Login(user_unchecked, author) => (Request::Login(user_unchecked), author),
            RequestRaw::Register(user_unchecked, author) => (Request::Register(user_unchecked), author),
            RequestRaw::GetWaitingMessagesAuto(author) => (Request::GetWaitingMessagesAuto, author),
            RequestRaw::Unknown(author) => (Request::Unknown, author),
        };

        let mut message = ImplementedMessage::new();
        let content = Content::with_data(request.to_ron()?);
        let content_buff = content.into_bytes();

        // Recipient of Request will always be a server.
        let message_kind = MessageKind::Request;
        let recipients = vec![SERVER_USERNAME.to_string().clone()];
        let file_name = None;

        let metadata = MetaData::new(&content_buff, message_kind, author, SERVER_ID, recipients, file_name)?;
        message.set_metadata(metadata);

        message.set_content(Content::from_bytes(content_buff)?);

        let end_data = Packet::new(PacketKind::End, Bytes::new());
        message.set_end_data(end_data);

        Ok(message)  
    }
}