pub mod message;
pub mod user;
pub mod config;

pub use message::{Content, MetaData, MessageKind, Request, RequestRaw};

pub type ImplementedMessage = library::message::Message<MetaData, Content>;

use library::packet::Packet;
