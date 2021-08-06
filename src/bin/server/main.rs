use std::io::Write;
use std::net::{TcpListener};
use std::fs;

use library::prelude::*;

/// Server now listens for only one message, then the process will end.
fn main() {

    let socket = format!("{}:{}", ADDR, PORT);
    let listener = TcpListener::bind(socket).unwrap();
    println!("Listening for connections.");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let msg = Message::receive(&mut stream);
                match msg.kind() {
                    MessageKind::File => {
                        let mut file = fs::File::create(msg.metadata().file_name().unwrap()).unwrap();
                        file.write(&msg.content().clone()).unwrap();
                    },
                    MessageKind::Text => {
                        println!("{:?} \n Content: {}", &msg, String::from_buff(msg.clone().content()));
                    },
                    _ => {},
                }
            },
            Err(_) => todo!(),
        }
    }
}
