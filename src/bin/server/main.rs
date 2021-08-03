extern crate lib;
use std::net::{TcpListener};

use lib::*;


fn main() {

    let mtd = MetaData::new(MessageKind::Text, 5, 156468, 789153,
         vec!["Fred".to_owned(), "John".to_owned(), "Lucy".to_owned()],
          None);

    println!("{}", mtd.to_ron_pretty(None));

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
