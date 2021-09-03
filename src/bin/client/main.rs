use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{net::TcpStream, thread};
use std::sync::mpsc;


/// Module used to get and process user input through commands.
mod command;
use command::CommandRaw;
use library::bytes::{FromBytes, IntoBytes};
use library::error::NetCommsError;
use library::prelude::{FromRon, IntoMessage, IntoRon};
use shared::message::ServerReply;
use shared::user::user::UserLite;
use shared::{ImplementedMessage, MessageKind, RequestRaw};
use shared::config::{ADDR, PORT};
use shared::user::User;

mod client;


// ERROR HANDLING
fn main() -> Result<(), NetCommsError> {

    let socket = format!("{}:{}", ADDR, PORT);
    let user = get_user()?;
    let (waiting_messages_transmitter, _waiting_messages_receiver) =
    mpsc::channel::<ImplementedMessage>();
    let _ = get_waiting_messages(user.clone(), socket.clone(), waiting_messages_transmitter);

    loop {
        let cmd_raw = CommandRaw::get(Some("send <(recipient_1, recipient_2, ..., recipient_n)> <content> \n"));
        let cmd = cmd_raw.process(user.clone()).unwrap();
        let message = cmd.into_message()?;


        match TcpStream::connect(&socket) {
            Ok(mut stream) => {
                if let Some(path) = message.metadata().file_name() {
                    ImplementedMessage::send_file(&mut stream, Path::new(&path))?;
                } else {
                    message.send(&mut stream)?;
                }
            },            
            Err(e) => {
                println!("{}", e);
            },
        };
    }
}


// Will be looping until the user had been received or until unrecoverable error.
fn get_user() -> Result<UserLite, NetCommsError> {
    let socket = format!("{}:{}", ADDR, PORT);
    // Get user by login or register. Only register works now.
    let user = UserLite::default_user();
    let cmd_raw = command::CommandRaw::get(Some("register <username> <password> <password>\nlogin <username> <password>\n".to_string()));
    let cmd = cmd_raw.process(user)?;
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
                        todo!();
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

fn get_waiting_messages(user: UserLite, socket: String, _mpsc_transmitter: Sender<ImplementedMessage>) -> JoinHandle<()> {

    thread::Builder::new().name("GetWaitingMessages".to_string()).spawn(move || {

        loop {
            // Need to solve error handling. Maybe another mpsc channel?
            let request = RequestRaw::GetWaitingMessagesAuto(user.clone());
            let message = request.into_message().unwrap();

            match TcpStream::connect(&socket) {
                Ok(mut stream) => {
                    message.send(&mut stream).unwrap();
                    loop {
                        let location = Path::new("D:\\stepa\\Documents\\Rust\\net_comms_logs\\client_logs");
                        match ImplementedMessage::receive(&mut stream, Some(location.to_path_buf())) {
                            Ok(message) => {

                                let metadata = message.metadata();
                                let message_kind = metadata.message_kind();         

                                let message_pretty = message.into_ron_pretty(None).unwrap();
                                let mut file = fs::File::create("received_message.ron").unwrap();
                                file.write_all(&message_pretty.into_buff()).unwrap();

                                // Why use multiple statements, when I can use one :D
                                println!("{author} [{datetime}]: {content}",
                                    author = message.metadata().author_username(),
                                    datetime = message.metadata().datetime_as_string(),
                                    content = match message_kind {
                                        MessageKind::File => format!("Received a file {name} at {location}",
                                                                        name = PathBuf::from(message.metadata().file_name().unwrap()).file_name().unwrap().to_string_lossy(),
                                                                        location = PathBuf::from(message.metadata().file_name().unwrap()).to_string_lossy()
                                                                    ),
                                        _ => String::from_buff(&message.content_move().into_buff()).unwrap()                                                        
                                    }) 
                            }
                            Err(_) => break,
                        }
                    }
                },
                Err(_) => todo!(),
            }
            thread::sleep(Duration::new(1, 0));
        }
    }).unwrap()
}
