/// Minimum value is 11, 8 for packet size, 2 for packet kind, and at least 1 for content. 
pub const MAX_PACKET_SIZE: usize = 8096;
/// Minimal size that is every packet guaranteed to have, 8 bytes are for its size and two for its kind.
pub const PACKET_DESCRIPTION_SIZE: usize = 10;
/// Maximum amount of bytes that a Packet can use for its content, its lower than MAX_PACKET_SIZE of PACKET_DESCRIPTION_SIZE.
pub static MAX_PACKET_CONTENT_SIZE: usize = MAX_PACKET_SIZE - PACKET_DESCRIPTION_SIZE;

pub const PORT: &str = "8000";
pub const ADDR: &str = "127.0.0.1";
pub const SERVER_ID: usize  = 0;
pub const DEFAULT_ID: usize  = 1;

// This will be later in client config file
pub const FILE_STORAGE: &'static str = "D:\\stepa\\Documents\\Rust\\net_comms\\sending_files_test";