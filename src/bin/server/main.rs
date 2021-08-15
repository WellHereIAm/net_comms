use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, mpsc};
use std::time::Duration;

use library::prelude::*;

mod config;

fn main() -> Result<(), NetCommsError> {

    let socket = format!("{}:{}", ADDR, PORT);
    let listener = TcpListener::bind(socket).unwrap();
    println!("Listening for connections.");

    let waiting_messages: Arc<Mutex<HashMap<usize, Vec<Message>>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut users = HashMap::new();
    users.insert("xxx".to_string(), User::new(1, "xxx".to_string(), "my_password".to_string())); // instead of id whole user struct should be used.
    let arc_users = Arc::new(Mutex::new(users));

    for stream in listener.incoming() {
        println!("Got connection.");
        match stream {
            Ok(mut stream) => {

                let waiting_messages = Arc::clone(&waiting_messages);
                let users = Arc::clone(&arc_users);

                let handle = thread::spawn(move || {
                    let msg = Message::receive(&mut stream).unwrap();
                    let author_id = msg.metadata().author_id();

                    match msg.metadata().message_kind() {
                        MessageKind::Text | MessageKind::File => {
                            // Many possibilities for errors
                            for recipient in msg.metadata().recipients() {
                                loop {
                                    match users.try_lock() {
                                        Ok(users_map) => {
                                            let user = users_map.get(&recipient).unwrap();
                                            let mut waiting_messages = waiting_messages.try_lock().unwrap();
                                            match waiting_messages.get_mut(&user.id()) {
                                                Some(messages) => messages.push(msg.clone()),
                                                None => {
                                                    let messages = vec![msg.clone()];
                                                    waiting_messages.insert(user.id(), messages);
                                                }
                                            }
                                            println!("{:?}", &waiting_messages);
                                            break;
                                        },
                                        Err(_) => todo!(),
                                    }
                                    
                                    thread::sleep(Duration::new(1, 0));
                                }
                            }                                                                       
                        },
                        MessageKind::Request => {
                            // Match request and handle id.
                        },
                        _ => {},
                    }

                });
                // let msg = Message::receive(&mut stream)?;
                // match msg.kind() {
                //     MessageKind::File => {},
                //     MessageKind::Text => {
                //         println!("Content: {}", String::from_buff(msg.clone().content())?);
                //     },
                //     _ => {},
                // }
                // println!("msg: {:?}", &msg);
            },
            Err(_) => todo!(),
        }
        println!("Handled connection.");
    }

    Ok(())
}


fn handle_connection(mut stream: TcpStream, mutex: MutexGuard<HashMap<usize, Vec<Message>>>) {
    let msg = Message::receive(&mut stream).unwrap();
    let author_id = msg.metadata().author_id();
    match msg.metadata().message_kind() {
        MessageKind::Text | MessageKind::File => {

        },
        MessageKind::Request => {
            // Handle request.
        },
        _ => {},
    }

}