use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{fs, io, thread};
use std::io::{Read, Write};
use std::sync::mpsc::{self, Receiver, Sender};

use nardol::prelude::{FromBytes, FromRon, IntoBytes, IntoMessage, ToRon};
use serde::{Serialize, Deserialize};
use ron::de;

use nardol::error::{NetCommsError, NetCommsErrorKind};
use shared::message::ServerReply;
use shared::{ImplementedMessage, MessageKind, RequestRaw};
use shared::user::UserLite;

use crate::command::{self, CommandRaw};


pub enum Output {
    Error(String),
    FromRun(String),
    FromUserInput(String),   
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    ip: String,
    port: u16,
    request_incoming_messages_timer: u64,
    save_location: PathBuf,
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

pub struct Client {
    user: UserLite,
    config: ClientConfig,
}

impl Client {
    
    pub fn new(config_location: &Path) -> Result<Self, NetCommsError> {
        Ok(Client {
            user: UserLite::default_user(),
            config: ClientConfig::new(config_location)?,
        })
    }

    pub fn run(mut self) -> Result<(), NetCommsError> {

        let (output_t, output_r) = mpsc::channel();
        let (waiting_messages_t, _waiting_messages_r) =
            mpsc::channel::<ImplementedMessage>();
        
        let socket = SocketAddr::from_str(&format!("{}:{}", self.config.ip, self.config.port)).unwrap();

        let user = Self::get_user(socket, self.user.clone())?;

        Self::output(output_r);
        // Self::start_input(output_t.clone());


        self.user = user;

        let _ = Self::get_waiting_messages(self.user.clone(),
                                                socket.clone(), 
                                                waiting_messages_t,
                                                self.config.request_incoming_messages_timer,
                                                self.config.save_location.clone(),
                                                output_t.clone());

        Self::process_user_input(socket.clone(), self.user.clone(), output_t.clone());

        Ok(())
    }

    fn output(output_r: Receiver<Output>) {
        
        thread::Builder::new().name("output".to_string()).spawn(move || {
            print!(">>> ");
            io::stdout().flush().unwrap();
            loop {
                match output_r.recv() {
                    Ok(output) => {
                        match output {
                            Output::FromRun(content) => {
                                if !content.is_empty() {
                                    print!("{}", content);
                                    print!("\n>>> ");
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

    fn _input(_output_t: Sender<Output>) {
        thread::Builder::new().name("input".to_string()).spawn(move|| {
            loop {
                let _cmd_raw = CommandRaw::get::<String>(None);
            }
        }).unwrap();
    }

    // Change string
    fn get_user(socket: SocketAddr, current_user: UserLite) -> Result<UserLite, NetCommsError> {
        
        let cmd_raw = command::CommandRaw::get(
            Some("register <username> <password> <password>\nlogin <username> <password>\n".to_string())
        );
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

    fn get_waiting_messages(user: UserLite, socket: SocketAddr, _mpsc_transmitter: Sender<ImplementedMessage>, 
                            request_incoming_messages_timer: u64,
                            location: PathBuf,
                            output_t: Sender<Output>) -> JoinHandle<()> {

        thread::Builder::new().name("GetWaitingMessages".to_string()).spawn(move || {
    
            loop {
                // Need to solve error handling. Maybe another mpsc channel?
                let request = RequestRaw::GetWaitingMessagesAuto(user.clone());
                let message = request.into_message().unwrap();
    
                match TcpStream::connect(&socket) {
                    Ok(mut stream) => {
                        message.send(&mut stream).unwrap();
                        loop {
                            match ImplementedMessage::receive(&mut stream, Some(location.clone())) {
                                Ok(message) => {
    
                                    let mut path = message.metadata().get_message_location(&location);
                                    path.push("message.ron");
                                    message.save(&path);
    
                                    let metadata = message.metadata();
                                    let message_kind = metadata.message_kind();         
    
                                    let message_pretty = message.to_ron_pretty(None).unwrap();
                                    let mut file = fs::File::create("received_message.ron").unwrap();
                                    file.write_all(&message_pretty.into_buff()).unwrap();


                                    let message = format!(
                                        "{author} [{datetime}]: {content}",
                                        author = message.metadata().author_username(),
                                        datetime = message.metadata().datetime_as_string(),
                                        content = match message_kind {
                                            MessageKind::File => format!("Received a file {name} at {location}",
                                                name = PathBuf::from(message.metadata().file_name().unwrap()).file_name().unwrap().to_string_lossy(),
                                                location = PathBuf::from(message.metadata().file_name().unwrap()).to_string_lossy()
                                                                        ),
                                            _ => String::from_buff(&message.content_move().into_buff()).unwrap()                                                        
                                    });

                                    output_t.send(Output::FromRun(message)).unwrap();
                                }
                                Err(_) => break,
                            }
                        }
                    },
                    Err(_) => todo!(),
                }
                thread::sleep(Duration::new(request_incoming_messages_timer, 0));
            }
        }).unwrap()
    }

    fn process_user_input(socket: SocketAddr, user: UserLite, _output_t: Sender<Output>) {

        loop {
            let cmd_raw = CommandRaw::get::<String>(None);
            let cmd = cmd_raw.process(user.clone()).unwrap();
            let message = cmd.into_message().unwrap();
    
    
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
}