//! Framework built on top of Tcp from [std::net] that eases development of software that need to communicate through network.
//!
//! This library mostly establishes protocols and ways how are data structured, sent and received through its [Packet](packet::Packet)
//! and [Message](message::Message) structs with some convenient methods and modules.

/// Contains [Bytes](bytes::Bytes) struct, wrapper around [Vec] of [u8] that is used throughout this library to hold data in byte format.
///
/// Also contains traits [FromBytes](bytes::FromBytes) and [IntoBytes](bytes::IntoBytes) that allow implementors convert from and into bytes.
pub mod bytes;

/// [Error type](std::error::Error) for this library.
pub mod error;

/// Contains [Message](message::Message) and traits that need to be implemented for data inside [Message](message::Message).
pub mod message;

/// Module containing [Packet](packet::Packet) and [PacketKind](packet::PacketKind).
///
/// Those structs are default building blocks in this library.
pub mod packet;

/// Module containing [FromRon](ron::FromRon) and [ToRon](ron::ToRon) traits.
///
/// Those traits have default methods so they do not need any work on implementation.
pub mod ron;

/// Re-export of all modules to ease the development.
pub mod prelude {
    pub use crate::bytes::{self, *};
    pub use crate::error::{self, *};
    pub use crate::message::{self, *};
    pub use crate::packet::{self, *};
    pub use crate::ron::{self, *};
}