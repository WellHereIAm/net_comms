mod packet_kind;
pub mod packet;

/// Re-export to ease the access to those structs.
pub use packet_kind::PacketKind;
pub use packet::Packet;