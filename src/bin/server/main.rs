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
    users.insert("xxx".to_string(), User::new(2, "xxx".to_string(), "my_password".to_string()));
    users.insert("Lucy".to_string(), User::new(3, "xxx".to_string(), "password".to_string()));
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
                                }
                            }                                                                       
                        },
                        MessageKind::Request => {
                            let request = Request::from_ron(&String::from_buff(msg.content()).unwrap()).unwrap();
                            match request {
                                Request::Login(user_unchecked) => todo!(),
                                Request::Register(user_unchecked) => {
                                    let username = user_unchecked.username;
                                    let password = user_unchecked.password;
                                    let mut users = users.try_lock().unwrap();
                                    match users.get(&username) {
                                        Some(_) => {
                                            let mut msg = Message::new().unwrap();
                                            let metadata = MetaData::new(MessageKind::SeverReply, 3,
                                                                                                    SERVER_ID,
                                                                                                    msg.metadata().author_id(),
                                                                                                    vec!["something for now".to_string()],
                                                                                                    None).unwrap();
                                            msg.set_metadata(metadata);

                                            let server_reply_kind = ServerReplyKind::Error("This username is already used.".to_string());
                                            msg.push_content(Packet::new(PacketKind::new_content(server_reply_kind.to_ron()
                                            .unwrap()
                                            .to_buff()
                                            .unwrap())));
                                    

                                            msg.set_end_data(Packet::new(PacketKind::End));

                                            msg.send(&mut stream).unwrap();
                                        }
                                        None => {
                                            let user = User::new(3, username, password);
                                            users.insert(user.username().clone(), user.clone());
                                            let mut msg = Message::new().unwrap();
                                            let metadata = MetaData::new(MessageKind::SeverReply, 3,
                                                                                                    SERVER_ID,
                                                                                                    msg.metadata().author_id(),
                                                                                                    vec!["something for now".to_string()],
                                                                                                    None).unwrap();
                                            msg.set_metadata(metadata);

                                            msg.push_content(Packet::new(PacketKind::new_content(user.to_ron().unwrap().to_buff().unwrap())));

                                            msg.set_end_data(Packet::new(PacketKind::End));

                                            msg.send(&mut stream).unwrap();
                                        },
                                    }
                                },
                                Request::GetWaitingMessages => todo!(),
                                Request::Unknown => todo!(),
                            }
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