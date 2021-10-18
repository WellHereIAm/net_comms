use chrono::{DateTime, Utc};

use rusqlite::{Connection, ToSql};
use rusqlite::types::ValueRef;

use serde::{Serialize, Deserialize};

use indoc::indoc;

use std::{fs, io, thread};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::mpsc::{Receiver, Sender};

use nardol::error::{NetCommsError, NetCommsErrorKind};
use nardol::ron::{FromRon, ToRon};
use nardol::message::Message;
use nardol::bytes::{Bytes, FromBytes, IntoBytes};
use nardol::message::IntoMessage;
use nardol::packet::{Packet, PacketKind};

use shared::message::{Content, MessageKind, MetaData, ServerReplyRaw};
use shared::user::{Password, User, UserLite, UserUnchecked};
use shared::{ImplementedMessage, Request};

#[path ="./sql/mod.rs"]
pub(crate) mod sql;
use sql::*;

use utils::input;

pub enum Output {
    Error(String),
    FromRun(String),
    FromUserInput(String),   
}

// Why the fuck fields can be accessed inside Server without being public?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
    pub maximum_active_connections: u16,
    pub save_location: PathBuf,
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

pub fn check_maximum_active_connections(max: u16,
                                        can_start: Receiver<bool>,
                                        allowance: Sender<bool>,
                                        finished: Receiver<bool>) {

    let mut number_of_active_connections = 0;
    thread::Builder::new().name("check_maximum_active_connections".to_string()).spawn(move || {
        loop {
            match can_start.try_recv() {
                Ok(_) => {
                    if number_of_active_connections < max {
                        allowance.send(true).unwrap();
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


pub fn output(output_r: Receiver<Output>) {

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
}

pub fn server_input(output_t: Sender<Output>) {
        
    thread::Builder::new().name("input".to_string()).spawn(move|| {
        loop {
            let input = input("").unwrap();
            // Later handle input.
            output_t.send(Output::FromUserInput(format!("input: {:?}", input))).unwrap();
        }
    }).unwrap();
}

pub fn create_listener(config: &ServerConfig) -> TcpListener {

    let socket = SocketAddrV4::new(ip(config), config.port);
    match TcpListener::bind(socket) {
        Ok(listener) => return  listener,
        Err(e) => panic!("Failed to create listener.\n{}", &e),
    }
}

pub fn ip(config: &ServerConfig) -> Ipv4Addr {
    match Ipv4Addr::from_str(&config.ip) {
        Ok(ip) => return ip,
        Err(_) => panic!("Failed to get an ip address.\nFailed to parse string from config to Ipv4Addr."),
    }
}

pub fn handle_connection(mut stream: TcpStream,
    can_start: Sender<bool>,
    allowance: &Receiver<bool>,
    finished: Sender<bool>,
    output: Sender<Output>,
    config: &ServerConfig,
    db_path: &Path) {

    let location = config.save_location.clone();
    let db_location = db_path.to_owned();

    loop {
        if let Err(e) = can_start.send(true) {
            output.send(Output::Error(format!("Failed to send request to create a new thread.\n{}", e))).unwrap();
        }

        let can_start_answer = match allowance.recv() {
            Ok(can_start_sent) => can_start_sent,
            Err(e) => {
                output.send(Output::Error(
                    format!("Failed to receive an answer to request to create a new thread.\n{}", e)
                )).unwrap();
                false
            },
        };

        if can_start_answer {
            thread::Builder::new().name("connection".to_string()).spawn(move || {

            let mut db_conn = Connection::open(db_location).unwrap();

            match Message::receive(&mut stream, Some(location.clone())) {

                Ok(message) => {                        

                    let metadata: MetaData = message.metadata();
                    let message_kind: MessageKind = metadata.message_kind();
                    // let mut location = metadata.get_message_location(&location);
                    // location.push("message.ron");
                    // message.save(&location);

                    match message_kind {
                        MessageKind::Text | MessageKind::File => {
                            let _ = insert_message_into_database(message, &mut db_conn);    
                        },
                        MessageKind::Request => {
                            // Maybe should create a database to store those requests as well?
                            receive_request(message, stream, &mut db_conn, output);
                        },
                        _ => {}
                    }
                },
                Err(e) => {
                    let err_content = format!(indoc!{
                        "
                        Failed to receive a Message,
                        in Server::handle_connection().
                        error:
                        {}
                        "
                    }, e);
                    output.send(Output::Error(err_content)).unwrap();
                },
            }

            if let Err(e) = finished.send(true) {
                eprintln!("{}", e);
            }
            }).unwrap(); // !!!!!

            break;
        }
    }
}

fn receive_request(message: ImplementedMessage,
                   stream: TcpStream, 
                   db_conn: &mut Connection, 
                   output: Sender<Output>) {  

    let metadata = message.metadata();
    let _end_data = message.end_data();

    let content = message.content_move();

    let author = UserLite::new(metadata.author_id(),
                                           metadata.author_username());

    let request = Request::from_ron(&String::from_buff(&content.into_buff())
                                        .unwrap())
                                        .unwrap();

    match request {
        Request::Register(user_unchecked) => {
            user_register(stream, db_conn, user_unchecked, output);
        },
        Request::Login(user_unchecked) => {
            user_login(stream, db_conn, user_unchecked, output).unwrap();
        },
        Request::GetWaitingMessagesAuto => {
            return_waiting_messages(stream, db_conn, author, output);
        },
        Request::Unknown => todo!(),
    }
}

fn user_register(mut stream: TcpStream,
                     db_conn: &mut Connection,
                     user_unchecked: UserUnchecked,
                     output: Sender<Output>) {

    let UserUnchecked {username, password} = user_unchecked;

    // Checks if that username already exists.
    match get_user_id_from_username(db_conn, &username) {
        Ok(_) => {
            let server_reply = ServerReplyRaw::Error(
                "This username already exists.".to_string(),
                UserLite::default_user(),
            );
            let message = server_reply.into_message().unwrap();
            message.send(&mut stream).unwrap();
        },
        Err(_) => {
            let id = get_available_id(db_conn);
            let user = User::new(id as u32, username, Password::new(password));

            insert_new_user(db_conn, &user);

            output.send(Output::FromRun(user.clone().to_ron_pretty(None).unwrap())).unwrap();
            let user_lite = UserLite::from_user(&user);

            let server_reply = ServerReplyRaw::User(user_lite, UserLite::default_user());
            let message = server_reply.into_message().unwrap();
            message.send(&mut stream).unwrap();
        },
    }
}

fn user_login(mut stream: TcpStream,
                  db_conn: &mut Connection,
                  user_unchecked: UserUnchecked,
                  _output: Sender<Output>) -> Result<(), ()> {

    let UserUnchecked {username, password} = user_unchecked;
    let provided_password = password;
                
    match get_user_id_from_username(db_conn, &username) {
        Ok(id) => {
            let correct_password = match get_user_password(db_conn, id) {
                Ok(password) => password,
                Err(_) => {
                    let server_reply = ServerReplyRaw::Error(
                        "Username does not exist.".to_string(),
                         UserLite::default_user(),
                    );
                    let message = server_reply.into_message().unwrap();
                    message.send(&mut stream).unwrap();
                    return Err(())
                },
            };
            let correct_password = Password::from_hash(correct_password);
            if correct_password.verify(provided_password) {
                let user_lite = UserLite::new(id as u32, username);
                let server_reply = ServerReplyRaw::User(user_lite, UserLite::default_user());
                let message = server_reply.into_message().unwrap();
                message.send(&mut stream).unwrap();
            } else {
                let server_reply = ServerReplyRaw::Error(
                    "Incorrect password.".to_string(),
                     UserLite::default_user(),
                );
                let message = server_reply.into_message().unwrap();
                message.send(&mut stream).unwrap();
            }                
        },
        Err(_) => {
            let server_reply = ServerReplyRaw::Error(
                format!("User with username: {} does not exist", username),
                UserLite::default_user(),
            );
            let message = server_reply.into_message().unwrap();
            message.send(&mut stream).unwrap();
        },
    }
    Ok(())
}

fn return_waiting_messages(mut stream: TcpStream,
                           db_conn: &mut Connection, 
                           author: UserLite,
                           _output: Sender<Output>) {

    let messages = match get_waiting_messages_ids(db_conn, author.id() as usize) {
        Ok(messages) => messages,
        Err(_) => Vec::new(),
    };

    for message_id in messages {
        let message = get_message(db_conn, message_id).unwrap();
        message.send(&mut stream).unwrap();
        delete_waiting_message(db_conn, author.id() as usize).unwrap();
    }
}

