// MAYBE CHANGE THIS LATER TO RON? PROBABLY NOT. BINARIES CONFIGS PROBABLY WILL BE IN RON.

/// Minimum value is 11, 8 for packet size, 2 for packet kind, and at least 1 for content. 
pub const MAX_PACKET_SIZE: usize = 2048;
pub const PORT: &str = "8000";
pub const ADDR: &str = "127.0.0.1";
pub const SERVER_ID: usize  = 0;
pub const DEFAULT_ID: usize  = 1;

// This will be later in client config file
pub const FILE_STORAGE: &'static str = "D:\\stepa\\Documents\\Rust\\net_comms\\sending_files_test";