/// Minimum value is 11, 8 for packet size, 2 for packet kind, and at least 1 for content. 
pub const MAX_PACKET_SIZE: u16 = 1024;

/// Minimal size that is every packet guaranteed to have, 2 bytes are for its size and 2 for its kind.
pub const PACKET_DESCRIPTION_SIZE: u16 = 4;

/// Maximum amount of bytes that a [Packet](crate::packet::Packet) can use for its content, its lower than [MAX_PACKET_SIZE] by [PACKET_DESCRIPTION_SIZE].
pub static MAX_PACKET_CONTENT_SIZE: u16 = MAX_PACKET_SIZE - PACKET_DESCRIPTION_SIZE;

/// Port that is used for connection.
pub const PORT: &str = "8000";
/// IP address that is used for connection.
pub const ADDR: &str = "127.0.0.1";
/// Id that is server using when communicating with clients.
pub const SERVER_ID: u32  = 0;
/// Username that is server using when communicating with clients.
pub const SERVER_USERNAME: &str = "SERVER";
/// User id that is used when client or server do not know the user that is sending or receiving a [Message](crate::message::Message).
/// Typical use is when sending a [login](crate::request::Request::Login) or [register](crate::request::Request::Register) request.
pub const UNKNOWN_USER_ID: u32  = 1;
/// User username that is used when client or server do not know the user that is sending or receiving a [Message](crate::message::Message).
/// Typical use is when sending a [login](crate::request::Request::Login) or [register](crate::request::Request::Register) request.
pub const UNKNOWN_USERNAME: &str = "UNKNOWN";
