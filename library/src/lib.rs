//! Framework that eases network communication using TCP protocol.
//! 
//! This is only a library which declared ways how are data structured inside the buffer containing sent or received data.
//! This framework is build on top of [std::net].


/// Module containing [FromBuffer] and [ToBuffer] traits.
///
/// Those traits are used throughout this library as they provide necessary functionality for given type to convert it to or from buffer,
/// which inside this library is always [Vec] of [u8].
pub mod buffer;

pub mod bytes;

/// [Error type](std::error::Error) for this library.
pub mod error;

/// Module containing [Message](message::Message) and other struct that are used inside it or with it.
pub mod message;

/// Module containing [Packet](packet::Packet) and other struct that are used inside it or with it.
pub mod packet;

/// Module containing [FromRon] and [ToRon] traits.
///
/// Those traits have default methods so they do not need any work on implementation.
pub mod ron;

/// Re-export of all modules to ease the development.
pub mod prelude {
    pub use crate::buffer::{self, *};
    pub use crate::error::{self, *};
    pub use crate::message::{self, *};
    pub use crate::packet::{self, *};
    pub use crate::ron::{self, *};
}