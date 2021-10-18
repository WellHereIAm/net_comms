use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{fs, io, thread};
use std::io::{Read, Write};
use std::sync::mpsc::{self, Receiver, Sender};

use nardol::prelude::{FromBytes, FromRon, IntoBytes, IntoMessage, ToRon};
use rusqlite::Connection;
use serde::{Serialize, Deserialize};
use ron::de;

use nardol::error::{NetCommsError, NetCommsErrorKind};
use shared::message::ServerReply;
use shared::{ImplementedMessage, MessageKind, RequestRaw};
use shared::user::UserLite;

use crate::command::{self, CommandRaw};


#[path ="./sql.rs"]
mod sql;
pub use sql::*;


pub enum Output {
    Error(String),
    FromRun(String),
    FromUserInput(String),   
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub ip: String,
    pub port: u16,
    pub request_incoming_messages_timer: u64,
    pub save_location: PathBuf,
}

impl ClientConfig {

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

pub fn get_user(socket: SocketAddrV4,
                current_user: UserLite,
                output_t: Sender<Output>) -> Result<UserLite, NetCommsError> {

    output_t.send(Output::FromRun(
        "Use register <username> <password> <password> or\nlogin <username> <password>\n".to_string()
    )).unwrap();       
    let cmd_raw = command::CommandRaw::get::<String>(None);
    let cmd = cmd_raw.process(current_user)?;
    let request = cmd.into_message()?;

    let location = Path::new("D:\\stepa\\Documents\\Rust\\net_comms_logs\\client_logs");
    match TcpStream::connect(socket.clone()) {
        Ok(mut stream) => {
            request.send(&mut stream)?;
            let msg = ImplementedMessage::receive(&mut stream, Some(location.to_path_buf()))?;
            let metadata = msg.metadata();
            let message_kind = metadata.message_kind();
            match message_kind {
                MessageKind::SeverReply => {
                    let server_reply = ServerReply::from_ron(&String::from_buff(&msg.content_move().into_buff())?)?;
                    if let ServerReply::User(user) = server_reply {
                        output_t.send(Output::FromRun("Successful login.".to_string())).unwrap();
                        return Ok(user);
                    } else {
                        println!("{:?}", server_reply);
                        panic!();
                    }
                }
                _ => {
                    todo!()
                }
                
            }
        },
        Err(_) => todo!(),
    }
}

pub fn output(output_r: Receiver<Output>) {
        
    thread::Builder::new().name("output".to_string()).spawn(move || {
        println!("Output started.");
        print!(">>> ");
        io::stdout().flush().unwrap();
        loop {
            match output_r.recv() {
                Ok(output) => {
                    match output {
                        Output::FromRun(content) => {
                            if !content.is_empty() {
                                println!("\n{}", content);
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

pub fn ip(config: &ClientConfig) -> Ipv4Addr {
    match Ipv4Addr::from_str(&config.ip) {
        Ok(ip) => return ip,
        Err(_) => panic!("Failed to get an ip address.\nFailed to parse string from config to Ipv4Addr."),
    }
}

pub fn get_waiting_messages(user: UserLite, socket: SocketAddrV4, _mpsc_transmitter: Sender<ImplementedMessage>, 
                            request_incoming_messages_timer: u64,
                            save_location: &Path,
                            db_path: &Path,
                            output_t: Sender<Output>) -> JoinHandle<()> {
    
    let db_location = db_path.to_owned();
    let save_location = save_location.to_owned();

    thread::Builder::new().name("GetWaitingMessages".to_string()).spawn(move || {

        let mut db_conn = Connection::open(db_location).unwrap();

        loop {
            // Need to solve error handling. Maybe another mpsc channel?
            let request = RequestRaw::GetWaitingMessagesAuto(user.clone());
            let message = request.into_message().unwrap();

            match TcpStream::connect(&socket) {           

                Ok(mut stream) => {
                    message.send(&mut stream).unwrap();
                    while let Ok(message)
                     = ImplementedMessage::receive(&mut stream, Some(save_location.clone())) {

                        let metadata = message.metadata();
                        let message_kind = metadata.message_kind();       

                        let message_out = format!(
                            "{author} [{datetime}]: {content}",
                            author = message.metadata().author_username(),
                            datetime = message.metadata().datetime_as_string(),
                            content = match message_kind {
                                MessageKind::File => format!("Received a file {name} at {location}",
                                    name = PathBuf::from(message.metadata().file_name().unwrap()).file_name().unwrap().to_string_lossy(),
                                    location = PathBuf::from(message.metadata().file_name().unwrap()).to_string_lossy()
                                                            ),
                                _ => String::from_buff(&message.content().into_buff()).unwrap()                                                        
                        });
                        
                        output_t.send(Output::FromRun(message_out)).unwrap();
                         
                        insert_message(&mut db_conn, message);
                    }
                },
                Err(_) => todo!(),
            }
            thread::sleep(Duration::new(request_incoming_messages_timer, 0));
        }
    }).unwrap()
}

pub fn process_user_input(socket: SocketAddrV4, user: UserLite, output_t: Sender<Output>) {

    loop {
        let cmd_raw = CommandRaw::get::<String>(None);
        let cmd = cmd_raw.process(user.clone()).unwrap();
        let message = cmd.into_message().unwrap();

        println!("{}", message.clone().to_ron_pretty(None).unwrap());

        match TcpStream::connect(&socket) {
            Ok(mut stream) => {
                if let Some(path) = message.metadata().file_name() {
                    ImplementedMessage::send_file(&mut stream, Path::new(&path)).unwrap();
                } else {
                    message.send(&mut stream).unwrap();
                }
            },            
            Err(e) => {
                println!("{}", e);
            },
        };
    }
}
