use crate::error::NetCommsError;
use crate::message::{Message, MessageKindType, MetaDataType, ContentType};

pub trait IntoMessage<'a, K, M, C> {
    fn into_message(self) -> Result<Message<K, M, C>, NetCommsError>
    where
        K: MessageKindType<'a>,
        M: MetaDataType<'a>,
        C: ContentType<'a, K, M, C>,;
    }