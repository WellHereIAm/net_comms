
mod metadata;
mod packet_kind;
mod packet;

/// Re-export to ease the access to those structs.
pub use metadata::MetaData;
pub use packet_kind::PacketKind;
pub use packet::Packet;