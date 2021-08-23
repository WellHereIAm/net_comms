use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;
use std::sync::{Arc, Mutex};

use library::prelude::*;

mod users;
mod server;

// ERROR HANDLING
fn main() -> Result<(), NetCommsError> {

    

    let socket = format!("{}:{}", ADDR, PORT);
    let listener = TcpListener::bind(socket).unwrap();
    println!("Listening for connections.");

    let waiting_messages: Arc<Mutex<HashMap<usize, Vec<Message>>>> = Arc::new(Mutex::new(HashMap::new()));


    let users = HashMap::new();
    let id = 2;
    dbg!(&users);
    let users = Arc::new(Mutex::new(users));
    let id = Arc::new(Mutex::new(id));

    for stream in listener.incoming() {
        println!("Got connection.");
        match stream {
            Ok(stream) => {
                handle_client(stream, Arc::clone(&users), Arc::clone(&id), Arc::clone(&waiting_messages));
            },
            Err(_) => todo!(),
        }
        println!("Handled connection.")
    }

    Ok(())
}


fn handle_client(mut stream: TcpStream,
                 users: Arc<Mutex<HashMap<String, User>>>,
                 id: Arc<Mutex<usize>>,
                 waiting_messages: Arc<Mutex<HashMap<usize, Vec<Message>>>>) {

    thread::Builder::new().name("client_thread".to_string()).spawn(|| {

        let location = Path::new("D:\\stepa\\Documents\\Rust\\net_comms\\server_logs");
        match Message::receive(&mut stream, location) {
            Ok(message) => {
                match message.kind() {
                    MessageKind::Text | MessageKind::File => {
                        for recipient in message.metadata().recipients() {
                            loop {
                                match users.try_lock() {
                                    Ok(users_map) => {
                                        let user = users_map.get(&recipient).unwrap();
                                        let mut waiting_messages = waiting_messages.try_lock().unwrap();
                                        match waiting_messages.get_mut(&user.id()) {
                                            Some(messages) => messages.push(message.clone()),
                                            None => {
                                                let messages = vec![message.clone()];
                                                waiting_messages.insert(user.id(), messages);
                                            }
                                        }
                                        println!("{:?}", &waiting_messages);
                                        break;
                                    },
                                    Err(e) => println!("{}", &e),
                                }
                            }
                        }             
                    },
                    MessageKind::Request => {
                        let author = User::new(message.metadata().author_id(), message.metadata().author_name(), "Dummy".to_string());
                        let request = Request::from_ron(&String::from_buff(message.content()).unwrap()).unwrap();
                        match request {
                            Request::Login(user_unchecked) => {
                                login(stream, users, user_unchecked);
                            },
                            Request::Register(user_unchecked) => {
                                register(stream, users, id, user_unchecked);
                            },
                            Request::GetWaitingMessages => {
                                return_waiting_messages(stream, author, waiting_messages);
                            },
                            Request::Unknown => todo!(),
                        }
                    },
                    _ => {}
                }
            },
            Err(e) => println!("{}", e),
        }

    }).unwrap();
}

fn register(mut stream: TcpStream,
            users: Arc<Mutex<HashMap<String, User>>>,
            id: Arc<Mutex<usize>>,
            user_unchecked: UserUnchecked) {

    let username = user_unchecked.username;
    let password = user_unchecked.password;

    let mut users = users.try_lock().unwrap();
    match users.get(&username) {
        Some(_) => {

            let mut message = Message::new().unwrap();
            let server_reply = ServerReply::Error("This username already exists.".to_string());
            let content = server_reply.to_ron().unwrap().to_buff().unwrap();
            let author = User::new(SERVER_ID, SERVER_USERNAME.to_string(), "None".to_string());
            let metadata = MetaData::new(&content, MessageKind::SeverReply,
                                                  author, UNKNOWN_USER_ID, vec![UNKNOWN_USER_ID.to_string()],
                                         None).unwrap();
            let end_data = Packet::new(PacketKind::End);

            message.set_metadata(metadata);
            for packet in Message::split_to_max_packet_size(content) {
                message.push_content(Packet::new(PacketKind::new_content(packet)));
            }
            message.set_end_data(end_data);

            message.send(&mut stream).unwrap();
        },
        None => {

            let mut id = id.lock().unwrap();
            let user = User::new(*id, username, password); // Need to change ids dynamically.
            *id += 1;
            users.insert(user.username().clone(), user.clone());

            let mut message = Message::new().unwrap();
            let server_reply = ServerReply::User(user.clone());
            let content = server_reply.to_ron().unwrap().to_buff().unwrap();
            let metadata = MetaData::new(&content, MessageKind::SeverReply,
                                                 user.clone(), UNKNOWN_USER_ID, vec![UNKNOWN_USER_ID.to_string()],
                                                 None).unwrap();
            let end_data = Packet::new(PacketKind::End);

            message.set_metadata(metadata);
            for packet in Message::split_to_max_packet_size(content) {
                message.push_content(Packet::new(PacketKind::new_content(packet)));
            }
            message.set_end_data(end_data);

            message.send(&mut stream).unwrap();
        },
    }
}

fn login(mut stream: TcpStream,
         users: Arc<Mutex<HashMap<String, User>>>,
         user_unchecked: UserUnchecked) {

    let username = user_unchecked.username;
    let password = user_unchecked.password;

    let users = users.try_lock().unwrap();
    match users.get(&username) {
        Some(user) => {
            if user.password() == password {
                let mut message = Message::new().unwrap();
                let server_reply = ServerReply::User(user.clone());
                let content = server_reply.to_ron().unwrap().to_buff().unwrap();
                let author = User::new(SERVER_ID, SERVER_USERNAME.to_string(), "None".to_string());
                let metadata = MetaData::new(&content, MessageKind::SeverReply,
                                                    author, UNKNOWN_USER_ID, vec![UNKNOWN_USER_ID.to_string()],
                                            None).unwrap();
                let end_data = Packet::new(PacketKind::End);

                message.set_metadata(metadata);
                for packet in Message::split_to_max_packet_size(content) {
                    message.push_content(Packet::new(PacketKind::new_content(packet)));
                }
                message.set_end_data(end_data);

                message.send(&mut stream).unwrap();
            }
        },
        None => {
            let mut message = Message::new().unwrap();
            let server_reply = ServerReply::Error("This username does not exists.".to_string());
            let content = server_reply.to_ron().unwrap().to_buff().unwrap();
            let author = User::new(SERVER_ID, SERVER_USERNAME.to_string(), "None".to_string());
            let metadata = MetaData::new(&content, MessageKind::SeverReply,
                                                  author, UNKNOWN_USER_ID, vec![UNKNOWN_USER_ID.to_string()],
                                         None).unwrap();
            let end_data = Packet::new(PacketKind::End);

            message.set_metadata(metadata);
            for packet in Message::split_to_max_packet_size(content) {
                message.push_content(Packet::new(PacketKind::new_content(packet)));
            }
            message.set_end_data(end_data);

            message.send(&mut stream).unwrap();
        },
    }
    
}

fn return_waiting_messages(mut stream: TcpStream,
                           author: User, 
                           waiting_messages: Arc<Mutex<HashMap<usize, Vec<Message>>>>) {

    let mut waiting_messages = waiting_messages.lock().unwrap();
    match waiting_messages.get_mut(&author.id()) {
        Some(messages) => {
            let messages_length = messages.len();
            for index in 0..messages_length {
                let mut message = messages.remove(index); 

                let mut metadata = message.metadata();
                metadata.set_author_id(SERVER_ID);
                metadata.set_recipients(vec![author.username()]);
                metadata.set_recipient_id(author.id());
                message.set_metadata(metadata);

                match message.metadata().file_name() {
                    Some(_) => message.send_file(&mut stream).unwrap(),                    
                    None => message.send(&mut stream).unwrap(),
                }
            }
        },
        None => {},
    }
}
