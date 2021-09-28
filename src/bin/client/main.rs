use std::path::PathBuf;
use std::str::FromStr;

mod command;
use nardol::error::NetCommsError;
use utils::input;

use crate::client::Client;

mod client;

// ERROR HANDLING
fn main() -> Result<(), NetCommsError> {

    // D:\\stepa\\Documents\\Rust\\net_comms\\src\bin\\client\\client_config.ron
    // C:\Documents\Rust\net_comms\src\bin\client\client_config.ron
    let config_location = get_config_location();

    let client = Client::new(&config_location)?;
    client.run()?;

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
