
/// All kinds of errors that can occur in and while using this library.
#[derive(Debug)]
pub enum NetCommsErrorKind {
    /// Used if given command does not exist.
    UnknownCommand,   
    /// Used if given command is not a valid command -- does not have proper structure or is missing its parts.
    InvalidCommand,

    /// Used if packet kind was not expected or a function or a method was used on wrong [PacketKind](crate::packet::PacketKind) variant.
    InvalidPacketKind, 

    /// Used if serializing struct with [serde] fails.
    SerializingFailed,
    /// Used if deserializing struct with [serde] fails.
    DeserializingFailed,

    /// Used if buffer size is not valid for the operation, usually used in implementations of [FromBuffer](crate::buffer::FromBuffer).
    InvalidBufferSize,

    /// Used if an error occurs during writing to [TcpStream](std::net::TcpStream).
    WritingToStreamFailed,
    /// Used if an error occurs during reading from [TcpStream](std::net::TcpStream).
    ReadingFromStreamFailed,

    /// Used if an error occurs while trying to open a file.
    OpeningFileFailed,
    /// Used if an error occurs while trying to write to a file.
    WritingToFileFailed,
    /// Used if an error occurs while trying to read from a file.
    ReadingFromFileFailed,

    /// Used if [Message](crate::message::Message) [MetaData](crate::packet::MetaData) are not complete.
    IncompleteMetaData,

    /// Wrapper around every error not originating from this library, used if there is a need to use
    /// produced error directly.
    OtherSource(Box<dyn std::error::Error>),
}

impl std::fmt::Display for NetCommsErrorKind {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NetCommsErrorKind")
    }    
}
