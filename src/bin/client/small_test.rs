use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{net::TcpStream, thread};
use std::sync::mpsc;


/// Module used to get and process user input through commands.
mod command;
use command::CommandRaw;
use library::bytes::{Bytes, FromBytes, IntoBytes};
use library::error::NetCommsError;
use library::prelude::{FromRon, IntoMessage, IntoRon, Packet};
use shared::message::ServerReply;
use shared::{Content, ImplementedMessage, MessageKind, MetaData, RequestRaw};
use shared::config::{ADDR, PORT};
use shared::user::User;

mod client;


fn main() -> Result<(), NetCommsError> {

    let content = Content::with_data("Hello, how are you?".to_string());
    let content_bytes = content.into_bytes();

    let metadata = MetaData::new_empty()?;
    let mut metadata = metadata.with_content_length(content_bytes.len());
    metadata.set_recipients(vec!["Francis".to_string(), "Lucy".to_string()]);

    let mut message = ImplementedMessage::new();
    message.set_metadata(metadata);
    message.set_content(Content::with_data("Hello, how are you?".to_string()));
    message.set_end_data(Packet::new(library::prelude::PacketKind::End, Bytes::from_arr([1, 5, 8, 4])));

    println!("{:?}", message);


    let socket = format!("{}:{}", ADDR, PORT);

    let mut stream = TcpStream::connect(&socket).unwrap();

    message.send(&mut stream).unwrap();

    Ok(())
}