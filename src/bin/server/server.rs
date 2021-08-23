use std::collections::HashMap;
use std::{fs, thread};
use std::io::Read;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Receiver, Sender};

use library::config::UNKNOWN_USER_ID;
use serde::{Serialize, Deserialize};
use ron::de;


use library::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port_start: usize,
    pub port_end: usize,
    pub maximum_active_connections: usize,
    pub save_location: PathBuf,
}

impl ServerConfig {

    pub fn new(config_location: &Path) -> Result<Self, NetCommsError> {

        match fs::File::open(config_location) {
            Ok(mut file) => {
                let mut buffer = String::new();
                if let Err(_) = file.read_to_string(&mut buffer) {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::ReadingFromFileFailed,
                        None));
                } 

                match de::from_str(&buffer) {
                    Ok(server_settings) => return Ok(server_settings),
                    Err(e) => Err(NetCommsError::new(
                        NetCommsErrorKind::DeserializingFailed,
                        Some(format!("Deserializing of given RON to ServerSettings struct failed.\n{:?}", e)))),
                }

            },
            Err(_) => return Err(NetCommsError::new(
                NetCommsErrorKind::OpeningFileFailed,
                None)),
        }
    }    
}

# [derive(Debug)]

pub struct Server {
   users: Arc<Mutex<HashMap<String, User>>>,
   ids: Arc<Mutex<Vec<usize>>>,
   waiting_messages: Arc<Mutex<HashMap<usize, Vec<Message>>>>,
   number_of_active_connections: Arc<Mutex<usize>>, 
   connection_handled: Receiver<bool>,
   connection_handled_sender: Sender<bool>, 
   config: ServerConfig,
}

impl Server {
    
    pub fn new(config_location: &Path) -> Result<Self, NetCommsError> {

        let users = Arc::new(Mutex::new(HashMap::new()));
        let ids = Arc::new(Mutex::new(vec![UNKNOWN_USER_ID + 1]));
        let waiting_messages = Arc::new(Mutex::new(HashMap::new()));
        let number_of_active_connections = Arc::new(Mutex::new(0));

        let (connection_handled_sender, connection_handled) = mpsc::channel::<bool>();

        let config = ServerConfig::new(config_location)?;

        let server = Server {
            users, 
            ids,
            waiting_messages,
            number_of_active_connections,
            connection_handled,
            connection_handled_sender,
            config,
        };

        Ok(server)
    }

    pub fn create_listener(&self) -> TcpListener {

        let socket = SocketAddrV4::new(self.ip(), self.ports().next().unwrap() as u16);
        match TcpListener::bind(socket) {
            Ok(listener) => return  listener,
            Err(e) => panic!("Failed to create listener.\n{}", &e),
        }
    }

    pub fn run(server: Arc<Mutex<Server>>) -> Result<(), NetCommsError> {

        println!("Starting server...");
        
        let server = Arc::clone(&server);
        thread::Builder::new().name("listener_thread".to_string()).spawn(move || {

            let server_guard = server.lock().unwrap();
            let listener = server_guard.create_listener();
            drop(server_guard);

            loop {
                match listener.accept() {
                    Ok((stream, _socket_addr)) => {
                        let server_guard = server.lock().unwrap();
                        Self::handle_connection(&server_guard, stream);
                    },
                    Err(e) => eprintln!("{}", e),
                };

                let server_guard = server.lock().unwrap();

                if let Ok(value) = server_guard.connection_handled.try_recv() {
                    if value {
                        let mut number_of_active_connections_guard = match server_guard.number_of_active_connections.lock() {
                            Ok(guard) => guard,
                            Err(poisoned) => poisoned.into_inner(),
                        };

                        *number_of_active_connections_guard -= 1;
                    }
                }
            }
        }).unwrap();

        println!("Server started.");
        Ok(())
    }

    fn handle_connection(&self, mut stream: TcpStream) {

        let users = Arc::clone(&self.users);
        let ids = Arc::clone(&self.ids);
        let waiting_messages = Arc::clone(&self.waiting_messages);
        let location = self.config.save_location.clone();

        let mut number_of_active_connections_guard = match self.number_of_active_connections.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let client_thread_sender = self.connection_handled_sender.clone();

        if *number_of_active_connections_guard < self.config.maximum_active_connections {
            *number_of_active_connections_guard += 1;
            drop(number_of_active_connections_guard);

            thread::Builder::new().name("client_thread".to_string()).spawn(move || {
                match Message::receive(&mut stream, location.as_path()) {
                    Ok(message) => {
                        match message.kind() {
                            MessageKind::Text | MessageKind::File => {
                                let _non_existent_recipients = Self::receive_user_to_user_message(message, waiting_messages, users);      
                            },
                            MessageKind::Request => {
                                Self::receive_request(message, stream, waiting_messages, users, ids);
                            },
                            _ => {}
                        }
                    },
                    Err(_) => todo!(),
                }

                if let Err(e) = client_thread_sender.send(true) {
                    eprintln!("{}", e);
                }
            }).unwrap(); // !!!!!
        }

        
    }

    fn receive_user_to_user_message(message: Message,
                                    waiting_messages: Arc<Mutex<HashMap<usize, Vec<Message>>>>,
                                    users: Arc<Mutex<HashMap<String, User>>>) -> Vec<String> {

        let mut non_existent_recipients = Vec::new();
                            
        for recipient in message.metadata().recipients() {

            let users_guard = match users.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };

            match users_guard.get(&recipient) {
                Some(user) => {
                    let mut waiting_messages_guard = match waiting_messages.lock() {
                        Ok(guard) => guard,
                        Err(poisoned) => poisoned.into_inner(),
                    };
                    match waiting_messages_guard.get_mut(&user.id()) {
                        Some(messages) => messages.push(message.clone()),
                        None => {
                            let messages = vec![message.clone()];
                            waiting_messages_guard.insert(user.id(), messages);
                        }
                    }
                },
                // Should do something with those, send them back - probably? Save as log?
                None => non_existent_recipients.push(recipient),
            }            
        }  
        
        if !non_existent_recipients.is_empty() {
            eprintln!("non_existent_recipients: {:?}", &non_existent_recipients);
        }
        non_existent_recipients
    }

    fn receive_request(message: Message,
                       stream: TcpStream, 
                       waiting_messages: Arc<Mutex<HashMap<usize, Vec<Message>>>>,
                       users: Arc<Mutex<HashMap<String, User>>>,
                       ids: Arc<Mutex<Vec<usize>>>) {

        let author = User::new(message.metadata().author_id(), message.metadata().author_name(), "Dummy".to_string());
        let request = Request::from_ron(&String::from_buff(message.content()).unwrap()).unwrap();

        match request {
            Request::Register(user_unchecked) => {
                Self::user_register(stream, users, ids, user_unchecked);
            },
            Request::Login(user_unchecked) => {
                Self::user_login(stream, users, user_unchecked);
            },
            Request::GetWaitingMessages => {
                Self::return_waiting_messages(stream, waiting_messages, author);
            },
            Request::Unknown => todo!(),
        }
    }

    fn user_register(mut stream: TcpStream,
                     users: Arc<Mutex<HashMap<String, User>>>,
                     ids: Arc<Mutex<Vec<usize>>>,
                     user_unchecked: UserUnchecked) {

        let username = user_unchecked.username;
        let password = user_unchecked.password;

        let mut users_guard = match users.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        match users_guard.get(&username) {
            Some(_) => {
                let server_reply = ServerReply::Error("This username already exists.".to_string());

                let message = Message::from_server_reply(server_reply).unwrap();
                if let Err(e) =  message.send(&mut stream) {
                    eprintln!("{}", e);
                }
            },
            None => {
                let mut ids_guard = match ids.lock() {
                    Ok(guard) => guard,
                    Err(poisoned) => poisoned.into_inner(),
                };

                let ids_length = ids_guard.len();
                let id = ids_guard.remove(ids_length - 1);
                if ids_guard.len() == 0 {
                    ids_guard.push(id + 1);
                }
                drop(ids_guard);
                
                let user = User::new(id, username, password); 
                
                users_guard.insert(user.username().clone(), user.clone());
                drop(users_guard);

                let server_reply = ServerReply::User(user);
                let message = Message::from_server_reply(server_reply).unwrap();
                if let Err(e) =  message.send(&mut stream) {
                    eprintln!("{}", e);
                }
            },
        }
    }

    fn user_login(mut stream: TcpStream, users: Arc<Mutex<HashMap<String, User>>>, user_unchecked: UserUnchecked) {

        let username = user_unchecked.username;
        let password = user_unchecked.password;

        let users_guard = match users.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        match users_guard.get(&username) {
            Some(user) => {
                if password == user.password() {
                    let server_reply = ServerReply::User(user.clone());
                    let message = Message::from_server_reply(server_reply).unwrap();
                    if let Err(e) =  message.send(&mut stream) {
                        eprintln!("{}", e);
                    }
                } else {
                    let server_reply = ServerReply::Error("Incorrect password.".to_string());
                    let message = Message::from_server_reply(server_reply).unwrap();
                    if let Err(e) =  message.send(&mut stream) {
                        eprintln!("{}", e);
                    }
                }
            },
            None => {
                let server_reply = ServerReply::Error(format!("User with username: {} does not exist", username));
                let message = Message::from_server_reply(server_reply).unwrap();
                if let Err(e) =  message.send(&mut stream) {
                    eprintln!("{}", e);
                }
            },
        }


    }

    fn return_waiting_messages(mut stream: TcpStream, waiting_messages: Arc<Mutex<HashMap<usize, Vec<Message>>>>, author: User) {

        let mut waiting_messages_guard = match waiting_messages.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        if let Some(messages) = waiting_messages_guard.get_mut(&author.id()) {
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
        }
    }

    pub fn ip(&self) -> Ipv4Addr {
        match Ipv4Addr::from_str(&self.config.ip) {
            Ok(ip) => return ip,
            Err(_) => panic!("Failed to get an ip address.\nFailed to parse string from config to Ipv4Addr."),
        }
    }

    pub fn ports(&self) -> RangeInclusive<usize> {
        self.config.port_start..=self.config.port_end
    }
}

# [test]
fn server_new() {
}