use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use library::prelude::*;

mod users;
mod server;
use server::Server;
use utils::input;

// ERROR HANDLING
fn main() -> Result<(), NetCommsError> {

    // D:\\stepa\\Documents\\Rust\\net_comms\\src\\bin\\server\\server_config.ron
    let config_location = get_config_location();

    let server: Arc<Mutex<Server>> = Arc::new(Mutex::new(Server::new(&config_location)?));

    Server::run(server)?;    

    // Controlling the server.
    loop {
        let _ = input("\n>>> ").unwrap();
    }
}

fn get_config_location() -> PathBuf {

    loop {
        let location = input("Enter server config location: \n>>> ").unwrap();
        
        match PathBuf::from_str(&location) {
            Ok(path) => return path,
            Err(_) => println!("Please enter valid server config location."),
        }
    }    
}
