use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{net::TcpStream, thread};
use std::sync::mpsc;

use library::buffer::{FromBuffer, ToBuffer};
use library::error::NetCommsError;
use library::config::{ADDR, PORT};
use library::message::{Message, MessageKind, ServerReply, ToMessage};
use library::ron::ToRon;
use library::pretty_structs::MessagePretty;
use library::user::User;

mod command;
use command::CommandRaw;
use shared::RequestRaw;

mod message;
mod client;


// ERROR HANDLING
fn main() -> Result<(), NetCommsError> {

    let socket = format!("{}:{}", ADDR, PORT);
    let user = get_user(&User::default())?;
    let (waiting_messages_transmitter, _waiting_messages_receiver) = mpsc::channel::<Message>();
    let _ = get_waiting_messages(user.clone(), socket.clone(), waiting_messages_transmitter);

    loop {
        let cmd_raw = CommandRaw::get(Some("send <(recipient_1, recipient_2, ..., recipient_n)> <content> \n"));
        let cmd = cmd_raw.process(&user).unwrap();
        let message = cmd.to_message()?;


        match TcpStream::connect(&socket) {
            Ok(mut stream) => {
                if let Some(_) = message.metadata().file_name() {
                    message.send_file(&mut stream)?;
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
fn get_user(user: &User) -> Result<User, NetCommsError> {
    let socket = format!("{}:{}", ADDR, PORT);
    // Get user by login or register. Only register works now.
    let user = user.clone();
    let cmd_raw = command::CommandRaw::get(Some("register <username> <password> <password>\nlogin <username> <password>\n".to_string()));
    let cmd = cmd_raw.process(&user)?;
    let request = cmd.to_message()?;


    let location = Path::new("D:\\stepa\\Documents\\Rust\\net_comms\\client_logs");
    match TcpStream::connect(socket.clone()) {
        Ok(mut stream) => {
            request.send(&mut stream)?;
            let msg = Message::receive(&mut stream, location)?;
            match msg.kind() {
                MessageKind::SeverReply => {
                    let server_reply = ServerReply::from_ron(&String::from_buff(msg.content_owned())?)?;
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

fn get_waiting_messages(user: User, socket: String, _mpsc_transmitter: Sender<Message>) -> JoinHandle<()> {

    thread::Builder::new().name("GetWaitingMessages".to_string()).spawn(move || {

        loop {
            // Need to solve error handling. Maybe another mpsc channel?
            let request = RequestRaw::GetWaitingMessagesAuto(user.clone());
            let message = request.to_message().unwrap();

            match TcpStream::connect(&socket) {
                Ok(mut stream) => {
                    message.send(&mut stream).unwrap();
                    loop {
                        let location = Path::new("D:\\stepa\\Documents\\Rust\\net_comms\\client_logs");
                        match Message::receive(&mut stream, location) {
                            Ok(message) => {

                                let message_pretty = MessagePretty::from_message(&message);
                                let mut file = fs::File::create("received_message.ron").unwrap();
                                file.write_all(&message_pretty.to_ron_pretty(None).unwrap().to_buff().unwrap()).unwrap();


                                // Why use multiple statements, when I can use one :D
                                println!("{author} [{datetime}]: {content}",
                                    author = message.metadata().author_username(),
                                    datetime = message.metadata().datetime_as_string(),
                                    content = match message.kind() {
                                        MessageKind::File => format!("Received a file {name} at {location}",
                                                                        name = PathBuf::from(message.metadata().file_name().unwrap()).file_name().unwrap().to_string_lossy(),
                                                                        location = PathBuf::from(message.metadata().file_name().unwrap()).to_string_lossy()
                                                                    ),
                                        _ => String::from_buff(message.content_owned()).unwrap()                                                        
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
