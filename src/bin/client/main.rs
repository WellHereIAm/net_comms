use std::net::TcpStream;

extern crate library;
use library::prelude::*;
fn main() -> Result<(), NetCommsError> {
    let user = User::new(25, "Štěpán".to_string(), "password".to_string());
    let cmd_raw = CommandRaw::get(Some("send <(recipient_1, recipient_2, ..., recipient_n)> <content> \n"));
    let cmd = cmd_raw.process(&user).unwrap();
    // println!("cmd in client: {:?}", &cmd);
    let msg = Message::from_command(cmd).unwrap();

    // D:\Software\Steam\steamapps\common\Apex Legends\paks\Win64\pc_all.opt.starpak 40
    // D:\Software\Steam\steamapps\common\Apex Legends\paks\Win64\pc_all.starpak 5

    
    // let config = ron::ser::PrettyConfig::new()
    //                                                 .with_depth_limit(4)
    //                                                 .with_decimal_floats(true); 
    // println!("{}", ron::ser::to_string_pretty(&msg, config).unwrap());

   let socket = format!("{}:{}", ADDR, PORT);

    match TcpStream::connect(socket) {
        Ok(mut stream) => {
            if let Some(file_name) = msg.metadata().file_name() {
                println!("Sending file: {}", file_name);
                msg.send_file(&mut stream)?;
            } else {
                msg.send(&mut stream)?;
            }
            // let time = std::time::SystemTime::now();
            // // msg.send(&mut stream);    
            // let duration = std::time::SystemTime::now().duration_since(time).unwrap().as_millis();
            // println!("Duration of sending: {}", duration / 1000);
        },            
        Err(e) => {
            println!("{}", e);
        },
    };

    Ok(())

    //let user = get_user();
    //println!("{:?}", user);  
}

// Later actually establish connection with the server.
// fn get_id(rqst: RequestKind) -> Result<[u8; 4], RequestErr> {

//     return Ok([1, 1, 1, 1]);

//     let socket = format!("{}:{}", ADDR, PORT);
    
//     let stream = match TcpStream::connect(socket) {
//         Ok(stream) => stream,
//         Err(_) => todo!(),
//     };
// }

// Returns User checked by server.
// There is an inner loop until function can return valid user.
// fn get_user() -> User {

//     use lib::RequestKind::GetId;
    
//     loop {
//         let mut is_first = true;
//         let cmd = get_user_cmd(is_first);
//         is_first = false;

//         let user: User;

//         match cmd.process_cmd() {
//             Ok(cmd) => {
//                 // logic to tell wrong usage of commands is in the process_cmd function. Like register name password without repeating password etc.
//                 match cmd {
//                     Command::Register(user_unchecked) => {

//                         match get_id(GetId(Box::new(RequestKind::Register(user_unchecked.clone())))) {
//                             Ok(id) => {
//                                 user = User::new(id, user_unchecked.username, user_unchecked.password);
//                                 return user;
//                             },
//                             Err(_) => {
//                                 println!("Error case of get_id using register")
//                             },
//                     }
//                 },
//                     Command::Login(user_unchecked) => {

//                         match get_id(GetId(Box::new(RequestKind::Login(user_unchecked.clone())))) {
//                             Ok(id) => {
//                                 user = User::new(id, user_unchecked.username, user_unchecked.password);
//                                 return user;
//                             },
//                             Err(_) => {
//                                 println!("Error case of get_id using login")
//                             },
//                     }
//                     },
//                     _ => {
//                         println!("Incorrect command.");
//                         continue;
//                     },
//                 }

//             },
//             Err(_) => {
//                 println!("Error case of process_cmd() inside of get_user()")
//             },
//         };
//     }        
// }

fn _get_user_cmd(is_first: bool) -> CommandRaw {

    loop {
        let msg: &str;
        if is_first {
            msg = 
                "
                Do you want to register or login?
                use \"login <username> <password>\" to login
                or \"register <username> <password> <password>\" to register\n";
        } else {
            msg = 
                "
                use \"login <username> <password>\" to login
                or \"register <username> <password> <password>\" to register\n";            
        }

        let cmd = CommandRaw::get(Some(msg));

        return cmd

    }
}
