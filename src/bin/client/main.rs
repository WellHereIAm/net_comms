
use std::{net::TcpStream, thread};

extern crate library;
use library::prelude::*;

fn main() -> Result<(), NetCommsError> {

    let socket = format!("{}:{}", ADDR, PORT);
    let user = get_user(&User::default())?;

    let connect_thread_handle = thread::spawn(|| {
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