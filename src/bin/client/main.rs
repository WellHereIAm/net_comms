use std::net::SocketAddrV4;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::mpsc;

mod command;
use nardol::error::NetCommsError;
use shared::ImplementedMessage;
use shared::user::UserLite;
use utils::input;

mod client;

use client::*;
use crate::client::open_database;

// ERROR HANDLING
fn main() -> Result<(), NetCommsError> {

    // D:\\stepa\\Documents\\Rust\\net_comms\\src\bin\\client\\client_config.ron
    // C:\Documents\Rust\net_comms\src\bin\client\client_config.ron
    let config_location = get_config_location();
    let config = ClientConfig::new(&config_location).unwrap();

    let mut db_path = config.save_location.clone();
    db_path.push("database.db");

    let (output_t, output_r) = mpsc::channel();
    let (waiting_messages_t, _waiting_messages_r) = mpsc::channel::<ImplementedMessage>();

    open_database(&db_path, output_t.clone()).unwrap();

    output(output_r);

    let socket = SocketAddrV4::new(ip(&config), config.port);

    let user = get_user(socket, UserLite::default_user(), output_t.clone()).unwrap();

    let handle = get_waiting_messages(user.clone(), socket, 
                                   waiting_messages_t,
                                                   config.request_incoming_messages_timer,
                                                   &config.save_location,
                                                   &db_path,
                                                   output_t.clone());

    process_user_input(socket, user, output_t);

    handle.join().unwrap();

    Ok(())
}

fn get_config_location() -> PathBuf {

    loop {
        let location = input("Enter client config location: \n>>> ").unwrap();
        
        match PathBuf::from_str(&location) {
            Ok(path) => {
                if path.is_file() {
                    return path
                }
            },
            Err(_) => println!("Please enter valid client config location."),
        }
    }    
}
