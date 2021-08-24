use std::collections::HashMap;
use std::{fs, io, thread};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Receiver, Sender};

use serde::{Serialize, Deserialize};
use indoc::indoc;

use library::prelude::*;
use utils::input;

pub enum Output {
    Error(String),
    FromRun(String),
    FromUserInput(String),   
}


// Why the fuck fields can be accessed inside Server without being public?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    ip: String,
    port: u16,
    maximum_active_connections: u16,
    save_location: PathBuf,
}

impl ToRon for ServerConfig {}
impl FromRon<'_> for ServerConfig {}

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

                match Self::from_ron(&buffer) {
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
   config: ServerConfig,
}

impl Server {
    
    pub fn new(config_location: &Path) -> Result<Self, NetCommsError> {

        let users = Arc::new(Mutex::new(HashMap::new()));
        let ids = Arc::new(Mutex::new(vec![UNKNOWN_USER_ID + 1]));
        let waiting_messages = Arc::new(Mutex::new(HashMap::new()));
        let config = ServerConfig::new(config_location)?;

        let server = Server {
            users, 
            ids,
            waiting_messages,
            config,
        };

        Ok(server)
    }

    pub fn create_listener(&self) -> TcpListener {

        let socket = SocketAddrV4::new(self.ip(), self.config.port);
        match TcpListener::bind(socket) {
            Ok(listener) => return  listener,
            Err(e) => panic!("Failed to create listener.\n{}", &e),
        }
    }

    pub fn run(self) -> Result<(), NetCommsError> {

        println!("[SERVER]: Starting server...");

        let (can_start_t, can_start_r) = mpsc::channel::<bool>(); 
        let (allowance_t, allowance_r) = mpsc::channel::<bool>(); 
        let (finished_t, finished_r) = mpsc::channel::<bool>(); 
        let (output_t, output_r) = mpsc::channel::<Output>();


        thread::Builder::new().name("output".to_string()).spawn(move || {
            print!(">>> ");
            io::stdout().flush().unwrap();
            loop {
                match output_r.recv() {
                    Ok(output) => {
                        match output {
                            Output::FromRun(content) => {
                                if !content.is_empty() {
                                    println!("\n[SERVER]: {}", content);
                                    print!(">>> ");
                                    io::stdout().flush().unwrap();
                                };
                            },
                            Output::Error(content) => {
                                if !content.is_empty() {
                                    println!("\n[ERROR]: {}", content);
                                    print!(">>> ");
                                    io::stdout().flush().unwrap();
                                };
                            },
                            Output::FromUserInput(content) => {
                                if content.is_empty() {
                                    print!(">>> ");
                                    io::stdout().flush().unwrap();
                                } else {
                                    println!("{}", content);
                                    print!(">>> ");
                                    io::stdout().flush().unwrap();
                                };
                            },
                        }                        
                    },
                    Err(e) => eprintln!("{}", e),
                }
            }
        }).unwrap();

        let output_t_input = output_t.clone();
        thread::Builder::new().name("input".to_string()).spawn(move|| {
            loop {
                let input = input("").unwrap();
                // Later handle input.
                output_t_input.send(Output::FromUserInput(format!("input: {}", input))).unwrap();
            }
        }).unwrap();

        
        Server::check_maximum_active_connections(self.config.maximum_active_connections,
                                                 can_start_r, allowance_t, finished_r);

        let listener = self.create_listener();

        let output_t_listener = output_t.clone();
        thread::Builder::new().name("listener".to_string()).spawn(move || {

            loop {
                match listener.accept() {
                    Ok((stream, _socket_addr)) => {
                        self.handle_connection(stream, can_start_t.clone(), &allowance_r, finished_t.clone(), output_t_listener.clone());
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }).unwrap();

        output_t.send(Output::FromRun("ServerStarted.".to_string())).expect("Server could not be started.");

        Ok(())
        
    }

    fn check_maximum_active_connections(max: u16, can_start: Receiver<bool>, allowance: Sender<bool>, finished: Receiver<bool>) {

        let mut number_of_active_connections = 0;
        thread::Builder::new().name("check_maximum_active_connections".to_string()).spawn(move || {
            loop {
                match can_start.try_recv() {
                    Ok(_) => {
                        if number_of_active_connections < max {
                            if let Err(e) = allowance.send(true) {
                                eprintln!("allowance send: {}", e);
                            }
                        }
                    },
                    Err(_) => {},
                }

                match finished.try_recv() {
                    Ok(_) => {
                        if number_of_active_connections != 0 {
                            number_of_active_connections -= 1;
                        }
                    },
                    Err(_) => {},
                }
            }
        }).unwrap();
    }
    fn handle_connection(&self, mut stream: TcpStream, can_start: Sender<bool>, allowance: &Receiver<bool>, finished: Sender<bool>,
                         output: Sender<Output>) {

        let users = Arc::clone(&self.users);
        let ids = Arc::clone(&self.ids);
        let waiting_messages = Arc::clone(&self.waiting_messages);
        let location = self.config.save_location.clone();

        loop {
            if let Err(e) = can_start.send(true) {
                output.send(Output::Error(format!("Failed to send request to create a new thread.\n{}", e))).unwrap();
            }

            let can_start_answer = match allowance.recv() {
                Ok(can_start_sent) => can_start_sent,
                Err(e) => {
                    output.send(Output::Error(format!("Failed to receive an answer to request to create a new thread.\n{}", e))).unwrap();
                    false
                },
            };

            if can_start_answer {
                thread::Builder::new().name("connection".to_string()).spawn(move || {
                    match Message::receive(&mut stream, location.as_path()) {
                        Ok(message) => {
                            match message.kind() {
                                MessageKind::Text | MessageKind::File => {
                                    let _ = Self::receive_user_to_user_message(message, waiting_messages, users, output.clone());      
                                },
                                MessageKind::Request => {
                                    Self::receive_request(message, stream, waiting_messages, users, ids, output);
                                },
                                _ => {}
                            }
                        },
                        Err(_) => todo!(),
                    }
    
                    if let Err(e) = finished.send(true) {
                        eprintln!("{}", e);
                    }
                }).unwrap(); // !!!!!

                break;
            }
        }
        
    }

    fn receive_user_to_user_message(message: Message,
                                    waiting_messages: Arc<Mutex<HashMap<usize, Vec<Message>>>>,
                                    users: Arc<Mutex<HashMap<String, User>>>,
                                    output: Sender<Output>) -> Vec<String> {

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
            output.send(Output::FromRun(format!("Non existent recipients of received message:\n{:?}", &non_existent_recipients))).unwrap();
        }
        non_existent_recipients
    }

    fn receive_request(message: Message,
                       stream: TcpStream, 
                       waiting_messages: Arc<Mutex<HashMap<usize, Vec<Message>>>>,
                       users: Arc<Mutex<HashMap<String, User>>>,
                       ids: Arc<Mutex<Vec<usize>>>,
                       output: Sender<Output>) {

        let author = User::new(message.metadata().author_id(), message.metadata().author_username(), "Dummy".to_string());
        let request = Request::from_ron(&String::from_buff(message.content()).unwrap()).unwrap();

        match request {
            Request::Register(user_unchecked) => {
                Self::user_register(stream, users, ids, user_unchecked, output);
            },
            Request::Login(user_unchecked) => {
                Self::user_login(stream, users, user_unchecked, output);
            },
            Request::GetWaitingMessagesAuto => {
                Self::return_waiting_messages(stream, waiting_messages, author, output);
            },
            Request::Unknown => todo!(),
        }
    }

    fn user_register(mut stream: TcpStream,
                     users: Arc<Mutex<HashMap<String, User>>>,
                     ids: Arc<Mutex<Vec<usize>>>,
                     user_unchecked: UserUnchecked,
                     output: Sender<Output>) {

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
                    let err_content = format!(indoc!{
                        "
                        Failed to send an error message:
                        \"This username already exists.\",
                        in Server::user_register() , case: user already exist.
                        error:
                        {}
                        "
                    }, e);
                    output.send(Output::Error(err_content)).unwrap();
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
                    let err_content = format!(indoc!{
                        "
                        Failed to send an correct User struct,
                        in Server::user_register().
                        error:
                        {}
                        "
                    }, e);
                    output.send(Output::Error(err_content)).unwrap();
                }
            },
        }
    }

    fn user_login(mut stream: TcpStream, users: Arc<Mutex<HashMap<String, User>>>, user_unchecked: UserUnchecked,
                  output: Sender<Output>) {

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
                        let err_content = format!(indoc!{
                            "
                            Failed to send an User struct,
                            in Server::user_login().
                            error:
                            {}
                            "
                        }, e);
                        output.send(Output::Error(err_content)).unwrap();
                    }
                } else {
                    let server_reply = ServerReply::Error("Incorrect password.".to_string());
                    let message = Message::from_server_reply(server_reply).unwrap();
                    if let Err(e) =  message.send(&mut stream) {
                        let err_content = format!(indoc!{
                            "
                            Failed to send an error message:
                            \"Incorrect password\"
                            in Server::user_login().
                            error:
                            {}
                            "
                        }, e);
                        output.send(Output::Error(err_content)).unwrap();
                    }
                }
            },
            None => {
                let server_reply = ServerReply::Error(format!("User with username: {} does not exist", username));
                let message = Message::from_server_reply(server_reply).unwrap();
                if let Err(e) =  message.send(&mut stream) {
                    let err_content = format!(indoc!{
                        "
                        Failed to send an error message:
                        \"User with username: {} does not exist\"
                        in Server::user_login().
                        error:
                        {}
                        "
                    }, username, e);
                    output.send(Output::Error(err_content)).unwrap();
                }
            }
        }
    }

    fn return_waiting_messages(mut stream: TcpStream, waiting_messages: Arc<Mutex<HashMap<usize, Vec<Message>>>>, author: User,
                               output: Sender<Output>) {

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
                        Some(_) => {
                            if let Err(e) = message.send_file(&mut stream) {
                                let err_content = format!(indoc!{
                                    "
                                    Failed to send a file,
                                    in Server::return_waiting_messages().
                                    error:
                                    {}
                                    "
                                }, e);
                                output.send(Output::Error(err_content)).unwrap();
                            }
                        },               
                        None => {
                            if let Err(e) = message.send(&mut stream) {
                                let err_content = format!(indoc!{
                                    "
                                    Failed to send back a message,
                                    in Server::return_waiting_messages().
                                    error:
                                    {}
                                    "
                                }, e);
                                output.send(Output::Error(err_content)).unwrap();
                            }
                        },
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
}

# [test]
fn server_new() {
}