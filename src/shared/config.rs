
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
