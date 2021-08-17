
use std::{net::TcpStream, thread};

extern crate library;
use library::prelude::*;

fn main() -> Result<(), NetCommsError> {

    let socket = format!("{}:{}", ADDR, PORT);
    let user = get_user(&User::default())?;
    
    let user_in_thread = user.clone();
    let socket_in_thread = socket.clone();
    let _ = thread::spawn(move || {
        let user = user_in_thread;

        let request = Request::GetWaitingMessages;
        // let metadata = MetaData::new(MessageKind::Request, 3,
        //                                                         user.id(),
        //                                                         SERVER_ID, vec!["SERVER".to_string()],
        //                                                         None).unwrap();
        let content = request.to_ron().unwrap();
        let end_data = Packet::new(PacketKind::End);

        let mut msg = Message::new().unwrap();
        // msg.set_metadata(metadata);
        msg.push_content(Packet::new(PacketKind::new_content(content.to_buff().unwrap())));
        msg.set_end_data(end_data);

        match TcpStream::connect(socket_in_thread) {
            Ok(mut stream) => {
                msg.send(&mut stream).unwrap();
                match Message::receive(&mut stream) {
                    Ok(msg) => {
                        match msg.kind() {
                            MessageKind::Text => println!("{}", String::from_buff(msg.content()).unwrap()),
                            // MessageKind::File => println!("Received file from {} saved.", msg.author_id()),
                            _ => {
                                todo!()
                            }
                        }
                        
                    },
                    Err(_) => todo!(),
                }
            },
            Err(_) => todo!(),
        }

        
        // Here I need to periodically connect to server and ask for received messages.
    });

    let cmd_raw = CommandRaw::get(Some("send <(recipient_1, recipient_2, ..., recipient_n)> <content> \n"));
    let cmd = cmd_raw.process(&user).unwrap();
    let msg = Message::from_command(cmd).unwrap();
    dbg!(&msg);


    match TcpStream::connect(socket) {
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

    Ok(())
}

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