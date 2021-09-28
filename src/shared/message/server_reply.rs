use nardol::bytes::Bytes;
use nardol::bytes::FromBytes;
use nardol::bytes::IntoBytes;
use nardol::prelude::FromRon;
use nardol::prelude::IntoMessage;
use nardol::prelude::ToRon;
use nardol::prelude::Packet;
use nardol::prelude::PacketKind;
use serde::{Serialize, Deserialize};
use ron::ser;
use ron::de;

use nardol::error::{NetCommsError, NetCommsErrorKind};

use crate::Content;
use crate::ImplementedMessage;
use crate::MessageKind;
use crate::MetaData;
use crate::config::SERVER_ID;
use crate::config::SERVER_USERNAME;
use crate::user::User;
use crate::user::UserLite;

/// Enum of all possible replies from server to client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerReply {
    /// Used when returning an error, [String] inside holds an error message.
    Error(String), // Later this string should be changed to use some kind of error enum, so client can recover from it.
    /// Used when there was a successful [Request::Register](crate::request::Request::Register) or [Request::Login](crate::request::Request::Login).
    User(UserLite),
}

impl ToRon for ServerReply {}
impl FromRon<'_> for ServerReply {}

pub enum ServerReplyRaw {
    /// Used when returning an error, [String] inside holds an error message.
    Error(String, UserLite), // Later this string should be changed to use some kind of error enum, so client can recover from it.
    /// Used when there was a successful [Request::Register](crate::request::Request::Register) or [Request::Login](crate::request::Request::Login).
    User(UserLite, UserLite),
}

impl IntoMessage<'_, MetaData, Content> for ServerReplyRaw {

    fn into_message(self) -> Result<ImplementedMessage, NetCommsError> {

        let (server_reply, recipient) = match self {
            ServerReplyRaw::Error(content, recipient) => {
                (ServerReply::Error(content), recipient)
            },
            ServerReplyRaw::User(user, recipient) => {
                (ServerReply::User(user), recipient)
            },
        };

        let mut message = ImplementedMessage::new();

        let content = Content::with_data(server_reply.to_ron()?);
        let content_buff = content.into_bytes();

        // Recipient of Request will always be a server.
        let message_kind = MessageKind::SeverReply;
        let recipients = vec![recipient.username()];
        let file_name = None;

        let metadata = MetaData::new(&content_buff, message_kind,
                                                UserLite::default_server(),
                                                recipient.id(), recipients,
                                                file_name)?;
        message.set_metadata(metadata);

        message.set_content(Content::from_bytes(content_buff)?);

        let end_data = Packet::new(PacketKind::End, Bytes::new());
        message.set_end_data(end_data);

        Ok(message)  
    }
}
