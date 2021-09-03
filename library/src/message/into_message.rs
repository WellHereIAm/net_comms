use crate::error::NetCommsError;
use crate::message::{Message, MetaDataType, ContentType};

pub trait IntoMessage<'a, M, C> {
    fn into_message(self) -> Result<Message<M, C>, NetCommsError>
    where
        M: MetaDataType<'a>,
        C: ContentType<'a, M, C>,;
    }