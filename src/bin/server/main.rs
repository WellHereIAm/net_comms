extern crate lib;
use std::net::{TcpListener, TcpStream};

use lib::*;

use utils::slice_to_array;


fn main() {

    let socket = format!("{}:{}", ADDR, PORT);
    let listener = TcpListener::bind(socket).unwrap();
    println!("Listening for connections.");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Got connection.");
                let msg = Message::new();
                Message::receive(&mut stream);
                println!("{:?}", msg);
            },
            Err(_) => todo!(),
        }
    }
}
