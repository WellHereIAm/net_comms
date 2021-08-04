use std::{collections::HashMap, io::Write, net::TcpStream, thread::sleep};

use utils::input;


extern crate library;
use library::prelude::*;
fn main() {

    let long_text = "But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure? On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection: he rejects pleasures to secure other greater pleasures, or else he endures pains to avoid worse pains. But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure? On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection: he rejects pleasures to secure other greater pleasures, or else he endures pains to avoid worse pains.But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure? On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection:";

    let command = Command::Send(MessageKind::Text, 1, vec!["Lucy".to_string()], "hello how are you".to_string().to_buff(), None);
    let msg = Message::from_command(command).unwrap();
    println!("{:?}", msg.metadata.clone().to_buff().len());
       
    let config = ron::ser::PrettyConfig::new()
                                                    .with_depth_limit(4)
                                                    .with_decimal_floats(true); 
   println!("{}", ron::ser::to_string_pretty(&msg, config).unwrap());

   let socket = format!("{}:{}", ADDR, PORT);

    match TcpStream::connect(socket) {
        Ok(mut stream) => {
            msg.send(&mut stream);    
        },            
        Err(e) => {
            println!("{}", e);
        },
    };

   //println!("{:?}", msg.metadata.clone().to_buff());
   //println!("{:?}", msg.add_info.clone());
   //println!("{:?}", msg.content.clone());
   //
   //
   

   //input("");

    //let user = get_user();
    //println!("{:?}", user);


    // Create empty value for every packet

    //x.push(EmptyPacket::new(10));
    //x.push(MetaDataPacket::new(10));

    // Loop till leave command is used. Later use more threads, one to process commands, other to show messages etc.
    //loop {
    //    let cmd = CommandRaw::get::<String>(None).process_cmd();
    //    match cmd {
    //        Ok(cmd) => {
    //            match cmd {
    //                Command::Register(_) => todo!(),
    //                Command::Login(_) => todo!(),
    //                Command::Yes => todo!(),
    //                Command::No => todo!(),
    //                Command::Send(..) => todo!(),
    //                Command::Unknown => todo!(),
    //            }
    //        },
    //        Err(e) => {
    //            println!("Error in main while processing command. {}", e)
    //        },
    //    }
    //    
    //}

    

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

fn get_user_cmd(is_first: bool) -> CommandRaw {

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
