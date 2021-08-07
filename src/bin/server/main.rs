use std::net::{TcpListener};

use library::prelude::*;

mod config;

/// Server now listens for only one message, then the process will end.
fn main() -> Result<(), NetCommsError> {

    let socket = format!("{}:{}", ADDR, PORT);
    let listener = TcpListener::bind(socket).unwrap();
    println!("Listening for connections.");

    for stream in listener.incoming() {
        println!("Got connection.");
        match stream {
            Ok(mut stream) => {
                let msg = Message::receive(&mut stream)?;
                match msg.kind() {
                    MessageKind::File => {},
                    MessageKind::Text => {
                        println!("{:?} \n Content: {}", &msg, String::from_buff(msg.clone().content())?);
                    },
                    _ => {},
                }
                println!("msg: {:?}", &msg);
            },
            Err(_) => todo!(),
        }
        println!("Handled connection.");
    }

    Ok(())
}
