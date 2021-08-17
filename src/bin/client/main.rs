
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{net::TcpStream, thread};
use std::sync::mpsc;

extern crate library;
use library::prelude::*;


// ERROR HANDLING
fn main() -> Result<(), NetCommsError> {

    let socket = format!("{}:{}", ADDR, PORT);
    let user = get_user(&User::default())?;

    let (waiting_messages_transmitter, waiting_messages_receiver) = mpsc::channel::<Message>();
    let _get_waiting_messages_handle = get_waiting_messages(user.clone(), socket.clone(), waiting_messages_transmitter);

    loop {
        let cmd_raw = CommandRaw::get(Some("send <(recipient_1, recipient_2, ..., recipient_n)> <content> \n"));
        let cmd = cmd_raw.process(&user).unwrap();
        let msg = Message::from_command(cmd).unwrap();
        dbg!(&msg);

        match TcpStream::connect(&socket) {
            Ok(mut stream) => {
                if let Some(file_name) = msg.metadata().file_name() {
                    println!("Sending file: {}", file_name);
                    msg.send_file(&mut stream)?;
                } else {
                    msg.send(&mut stream)?;
                }
            },            
            Err(e) => {
                println!("{}", e);
            },
        };

        loop {
            match waiting_messages_receiver.try_recv() {
                Ok(message) => println!("{}", String::from_buff(message.content()).unwrap()),
                Err(_) => break,
            }
        }
    }    
}


// Will be looping until the user had been received or until unrecoverable error.
fn get_user(user: &User) -> Result<User, NetCommsError> {
    let socket = format!("{}:{}", ADDR, PORT);
    // Get user by login or register. Only register works now.
    let mut user = user.clone();
    let cmd_raw = CommandRaw::get(Some("register <username> <password> <password>\nlogin <username> <password>\n".to_string()));
    let cmd = cmd_raw.process(&user)?;
    let request = Message::from_command(cmd)?;

    match TcpStream::connect(socket.clone()) {
        Ok(mut stream) => {
            request.send(&mut stream)?;
            let msg = Message::receive(&mut stream)?;
            match msg.kind() {
                MessageKind::SeverReply => {
                    user = User::from_ron(&String::from_buff(msg.content())?)?;
                    return Ok(user);
                }
                _ => {
                    todo!()
                }
                
            }
        },
        Err(_) => todo!(),
    }
} 

fn get_waiting_messages(user: User, socket: String, mpsc_transmitter: Sender<Message>) -> JoinHandle<()> {

    thread::Builder::new().name("GetWaitingMessages".to_string()).spawn(move || {
        // Need to solve error handling. Maybe another mpsc channel?
        let request = Request::GetWaitingMessages;
        let content = request.to_ron().unwrap().to_buff().unwrap();

        let message_kind = MessageKind::Request;
        let recipients = vec![SERVER_NAME.to_string()];

        let metadata = MetaData::new(&content, message_kind, user, SERVER_ID, recipients, None).unwrap();
        let end_data = Packet::new(PacketKind::End);

        let mut message = Message::new().unwrap();
        message.set_metadata(metadata);
        for packet in Message::split_to_max_packet_size(content) {
          message.push_content(Packet::new(PacketKind::new_content(packet)));
        }
       message.set_end_data(end_data);

        match TcpStream::connect(socket) {
            Ok(mut stream) => {
                message.send(&mut stream).unwrap();
                loop {
                    match Message::receive(&mut stream) {
                        Ok(message) => mpsc_transmitter.send(message).unwrap(),
                        Err(_) => break,
                    }
                }
            },
            Err(_) => todo!(),
        }

        thread::sleep(Duration::new(1, 0));
    }).unwrap()
}