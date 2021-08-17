
use std::net::TcpStream;

extern crate library;
use library::prelude::*;


fn main() -> Result<(), NetCommsError> {
    let user = User::new(25, "Štěpán".to_string(), "password".to_string());
    let cmd_raw = CommandRaw::get(Some("send <(recipient_1, recipient_2, ..., recipient_n)> <content> \n"));
    let cmd = cmd_raw.process(&user).unwrap();
    let msg = Message::from_command(cmd).unwrap();

   let socket = format!("{}:{}", ADDR, PORT);

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
