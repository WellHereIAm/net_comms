use std::path::PathBuf;
use std::str::FromStr;

use library::prelude::*;

mod users;
mod server;
use server::Server;
use utils::input;

// ERROR HANDLING
fn main() -> Result<(), NetCommsError> {

    // D:\\stepa\\Documents\\Rust\\net_comms\\src\\bin\\server\\server_config.ron
    let config_location = get_config_location();

    let server = Server::new(&config_location)?;
    server.run()?;

    loop {
        
    }
}

fn get_config_location() -> PathBuf {

    loop {
        let location = input("Enter server config location: \n>>> ").unwrap();
        
        match PathBuf::from_str(&location) {
            Ok(path) => {
                if path.is_file() {
                    return path
                }
            },
            Err(_) => println!("Please enter valid server config location."),
        }
    }    
}
