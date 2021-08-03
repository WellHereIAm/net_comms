extern crate lib;
use std::{io::Read, net::{TcpListener, TcpStream}};

use lib::*;

use utils::{input, slice_to_array};


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

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(mut stream) => {
    //             println!("Got connection.");
    //             let msg = Message::new();
    //             println!("Created empty message.");
    //             Message::receive(&mut stream);
    //             println!("Received message.");
    //             println!("{:?}", msg);
    //         },
    //         Err(_) => todo!(),
    //     }
    // }
    // input("");
}
