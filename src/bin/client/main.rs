extern crate lib;
use std::{collections::HashMap, net::TcpStream};



use lib::*;
use utils::input;


fn main() {   
    //let user = get_user();
    //println!("{:?}", user);
    let x = PacketRaw::get();

    struct PacketBlah {
        size: usize,
        kind: PacketKind,
        empty: EmptyPacket,
        metadata: MetaDataPacket,
        add_info: AddInfoPacket,
        content: ContentPacket,
        request: RequestPacket,
        unknown: UnknownPacket,
    }

    fn get<T: PacketType>() -> HashMap<String, Box<T>> {
        let mut hash = HashMap::new();
        hash.insert("PacketKind::Empty".to_string(), Box::new(EmptyPacket::new(10)));
        hash
    }

    let mut x: HashMap<PacketKind, T: PacketType> = HashMap::new();



    // Create empty value for every packet

    //x.push(EmptyPacket::new(10));
    //x.push(MetaDataPacket::new(10));

    println!("{:?}", x);

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
fn get_id(rqst: RequestKind) -> Result<[u8; 4], RequestErr> {

    return Ok([1, 1, 1, 1]);

    let socket = format!("{}:{}", ADDR, PORT);
    
    let stream = match TcpStream::connect(socket) {
        Ok(stream) => stream,
        Err(_) => todo!(),
    };
}

// Returns User checked by server.
// There is an inner loop until function can return valid user.
fn get_user() -> User {

    use lib::RequestKind::GetId;
    
    loop {
        let mut is_first = true;
        let cmd = get_user_cmd(is_first);
        is_first = false;

        let user: User;

        match cmd.process_cmd() {
            Ok(cmd) => {
                // logic to tell wrong usage of commands is in the process_cmd function. Like register name password without repeating password etc.
                match cmd {
                    Command::Register(user_unchecked) => {

                        match get_id(GetId(Box::new(RequestKind::Register(user_unchecked.clone())))) {
                            Ok(id) => {
                                user = User::new(id, user_unchecked.username, user_unchecked.password);
                                return user;
                            },
                            Err(_) => {
                                println!("Error case of get_id using register")
                            },
                    }
                },
                    Command::Login(user_unchecked) => {

                        match get_id(GetId(Box::new(RequestKind::Login(user_unchecked.clone())))) {
                            Ok(id) => {
                                user = User::new(id, user_unchecked.username, user_unchecked.password);
                                return user;
                            },
                            Err(_) => {
                                println!("Error case of get_id using login")
                            },
                    }
                    },
                    _ => {
                        println!("Incorrect command.");
                        continue;
                    },
                }

            },
            Err(_) => {
                println!("Error case of process_cmd() inside of get_user()")
            },
        };
    }


        
}

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
