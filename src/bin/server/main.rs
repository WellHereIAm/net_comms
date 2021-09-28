use std::thread;
use std::{path::PathBuf, sync::mpsc};
use std::str::FromStr;

use nardol::prelude::*;

mod server;
mod message;
use utils::input;

use server::*;

use server::sql::open_database;

// mod database;
// ERROR HANDLING
fn main() -> Result<(), NetCommsError> {

    // D:\\stepa\\Documents\\Rust\\net_comms\\src\\bin\\server\\server_config.ron
    // C:\Documents\Rust\net_comms\src\bin\server\server_config.ron
    let config_location = get_config_location();
    let config = ServerConfig::new(&config_location)?;

    let (can_start_t, can_start_r) = mpsc::channel::<bool>(); 
    let (allowance_t, allowance_r) = mpsc::channel::<bool>(); 
    let (finished_t, finished_r) = mpsc::channel::<bool>(); 
    let (output_t, output_r) = mpsc::channel::<Output>();

    // Creates output thread.
    output(output_r);
    output_t.send(Output::FromRun("Starting server...".to_string())).unwrap();

    let listener = create_listener(&config);

    let mut db_path = config.save_location.clone();
    db_path.push("database.db");

    open_database(&db_path, output_t.clone()).unwrap();
    server_input(output_t.clone());

    check_maximum_active_connections(config.maximum_active_connections.clone(),
                                         can_start_r, allowance_t, finished_r);

    let output_t_listener = output_t.clone();
    let listener_handle = thread::Builder::new().name("listener".to_string()).spawn(move || {
            loop {
                match listener.accept() {
                    Ok((stream, _socket_addr)) => {
                        handle_connection(stream,
                                          can_start_t.clone(), &allowance_r, finished_t.clone(),
                                          output_t_listener.clone(),
                                          &config,
                                          &db_path
                        );
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }).unwrap();

    output_t.send(Output::FromRun("Server started.".to_string())).unwrap();

    listener_handle.join().unwrap();
    Ok(())
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
