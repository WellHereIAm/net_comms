use std::net::{TcpListener};

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
                println!("{:?}", msg); 
                break;
            },
            Err(_) => todo!(),
        }
    }
}
