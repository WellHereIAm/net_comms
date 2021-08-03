extern crate lib;
use std::net::{TcpListener};

use lib::*;


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
