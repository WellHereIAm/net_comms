use crate::error::NetCommsError;
use crate::message::{Message, MetaDataType, ContentType};

/// Allows implementors convert `Self` into a [Message].
pub trait IntoMessage<'a, M, C> {

    /// Takes ownership os `self` and returns [Message] if successful.
    fn into_message(self) -> Result<Message<M, C>, NetCommsError>
    where
        M: MetaDataType<'a>,
        C: ContentType<'a, M, C>,;
}