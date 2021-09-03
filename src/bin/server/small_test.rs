use std::fs;
use std::io::Write;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{net::TcpStream, thread};
use std::sync::mpsc;


/// Module used to get and process user input through commands.
use library::bytes::{Bytes, FromBytes, IntoBytes};
use library::error::NetCommsError;
use library::prelude::{FromRon, IntoMessage, IntoRon, Packet};
use shared::message::ServerReply;
use shared::{Content, ImplementedMessage, MessageKind, MetaData, RequestRaw};
use shared::config::{ADDR, PORT};
use shared::user::User;

mod message;

fn main() -> Result<(), NetCommsError> {

    let socket = format!("{}:{}", ADDR, PORT);

    let mut listener = TcpListener::bind(socket).unwrap();

    for stream in listener.incoming() {
        let message = ImplementedMessage::receive(&mut stream.unwrap(),
         Some(PathBuf::from("D:\\stepa\\Documents\\Rust\\net_comms"))).unwrap();
        println!("content: {:?}", message.content_ref());
        println!("{:?}", message);
    }
   
    
    Ok(())
}