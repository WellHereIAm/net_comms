use chrono::{DateTime, Utc};
use library::bytes::{FromBytes, IntoBytes};
use library::prelude::{Bytes, IntoMessage, Packet, PacketKind};
use rusqlite::{Connection, ToSql};
use rusqlite::types::{Null, ToSqlOutput, ValueRef};
use serde::{Serialize, Deserialize};
use indoc::indoc;
use shared::{ImplementedMessage, Request};

use std::collections::HashMap;

use std::{fs, io, thread, vec};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Receiver, Sender};

use library::error::{NetCommsError, NetCommsErrorKind};
use library::ron::{FromRon, ToRon};
use library::message::Message;

use shared::message::{Content, MessageKind, MetaData, ServerReply, ServerReplyRaw};
use shared::user::{Password, User, UserLite, UserUnchecked, user};
use shared::config::{SERVER_ID, UNKNOWN_USERNAME, UNKNOWN_USER_ID};

use utils::input;

use crate::message;


enum Output {
    Error(String),
    FromRun(String),
    FromUserInput(String),   
}

enum DatabaseRequest {
    GetNewUserId,
    GetMessageId,
}

enum DatabaseAnswer {
    
}


// Why the fuck fields can be accessed inside Server without being public?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    ip: String,
    port: u16,
    maximum_active_connections: u16,
    save_location: PathBuf,
}

impl ToRon for ServerConfig {}
impl FromRon<'_> for ServerConfig {}

impl ServerConfig {

    pub fn new(config_location: &Path) -> Result<Self, NetCommsError> {

        match fs::File::open(config_location) {
            Ok(mut file) => {
                let mut buffer = String::new();
                if let Err(_) = file.read_to_string(&mut buffer) {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::ReadingFromFileFailed,
                        None));
                } 

                match Self::from_ron(&buffer) {
                    Ok(server_settings) => return Ok(server_settings),
                    Err(e) => Err(NetCommsError::new(
                        NetCommsErrorKind::DeserializingFailed,
                        Some(format!("Deserializing of given RON to ServerSettings struct failed.\n{:?}", e)))),
                }

            },
            Err(_) => return Err(NetCommsError::new(
                NetCommsErrorKind::OpeningFileFailed,
                None)),
        }
    }    
}

# [derive(Debug)]

pub struct Server {
   users: Arc<Mutex<HashMap<String, User>>>,
   ids: Arc<Mutex<Vec<u32>>>,
   waiting_messages: Arc<Mutex<HashMap<u32, Vec<ImplementedMessage>>>>,
   config: ServerConfig,
}

impl Server {
    
    pub fn new(config_location: &Path) -> Result<Self, NetCommsError> {

        let users = Arc::new(Mutex::new(HashMap::new()));
        let ids = Arc::new(Mutex::new(vec![UNKNOWN_USER_ID + 1]));
        let waiting_messages = Arc::new(Mutex::new(HashMap::new()));
        let config = ServerConfig::new(config_location)?;

        let server = Server {
            users, 
            ids,
            waiting_messages,
            config,
        };

        Ok(server)
    }

    pub fn run(self) -> Result<(), NetCommsError> {

        println!("[SERVER]: Starting server...");

        let (can_start_t, can_start_r) = mpsc::channel::<bool>(); 
        let (allowance_t, allowance_r) = mpsc::channel::<bool>(); 
        let (finished_t, finished_r) = mpsc::channel::<bool>(); 
        let (output_t, output_r) = mpsc::channel::<Output>();

        let mut db_path = self.config.save_location.clone();
        db_path.push("database.db");

        Self::database(&db_path).unwrap();
        Self::output(output_r);
        Self::input(output_t.clone());
    
        Server::check_maximum_active_connections(self.config.maximum_active_connections,
                                                 can_start_r, allowance_t, finished_r);

        let listener = self.create_listener();

        let output_t_listener = output_t.clone();
        thread::Builder::new().name("listener".to_string()).spawn(move || {
            loop {
                match listener.accept() {
                    Ok((stream, _socket_addr)) => {
                        self.handle_connection(stream,
                                               can_start_t.clone(), &allowance_r, finished_t.clone(), output_t_listener.clone(),
                                               &db_path);
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }).unwrap();

        output_t.send(Output::FromRun("ServerStarted.".to_string())).expect("Server could not be started.");

        Ok(())
        
    }

    fn database(db_path: &Path) -> Result<(), NetCommsError> {

        
        let db_conn = Connection::open(db_path).unwrap();
        // Check if this is a valid database. Now tables are always created.

        db_conn.execute(
            "CREATE TABLE users (
                id                  INTEGER NOT NULL,
                username            TEXT NOT NULL,
                password            TEXT NOT NULL,
                registration_date   TEXT NOT NULL,
                auth_token          TEXT DEFAULT NULL
            )", []).unwrap();

        // id should be later changed to AUTO INCREMENT
        db_conn.execute(
            "CREATE TABLE messages (
                id                  INTEGER PRIMARY KEY NOT NULL,
                kind                TEXT NOT NULL,
                length              INTEGER NOT NULL,
                datetime            TEXT NOT NULL,
                author_id           INTEGER,
                author_username     TEXT NOT NULL,
                recipient_id        INTEGER NOT NULL,
                file_name           TEXT,
                content             TEXT,
                end_data            TEXT
        )", []).unwrap();

        db_conn.execute(
            "CREATE TABLE message_recipients (
                message_id          INTEGER NOT NULL,
                recipient_id        INTEGER NOT NULL
        )", []).unwrap();

        db_conn.execute(
            "CREATE TABLE waiting_messages (
                message_id          INTEGER NOT NULL,
                recipient_id        INTEGER NOT NULL
            )", []).unwrap();

        // Last available id using integer as bool.
        db_conn.execute(
            "CREATE TABLE available_ids (
                id                  INTEGER NOT NULL,
                last                INTEGER NOT NULL
        )", []).unwrap();

        let first_id = 2;

        db_conn.execute("INSERT INTO available_ids (id, last) VALUES (?1, ?2)", [first_id, 1]).unwrap();

        Ok(())
    }

    fn output(output_r: Receiver<Output>) {

        thread::Builder::new().name("output".to_string()).spawn(move || {
            print!(">>> ");
            io::stdout().flush().unwrap();
            loop {
                match output_r.recv() {
                    Ok(output) => {
                        match output {
                            Output::FromRun(content) => {
                                if !content.is_empty() {
                                    println!("\n[SERVER]: {}", content);
                                    print!(">>> ");
                                    io::stdout().flush().unwrap();
                                };
                            },
                            Output::Error(content) => {
                                if !content.is_empty() {
                                    println!("\n[ERROR]: {}", content);
                                    print!(">>> ");
                                    io::stdout().flush().unwrap();
                                };
                            },
                            Output::FromUserInput(content) => {
                                if content.is_empty() {
                                    print!(">>> ");
                                    io::stdout().flush().unwrap();
                                } else {
                                    println!("{}", content);
                                    print!(">>> ");
                                    io::stdout().flush().unwrap();
                                };
                            },
                        }                        
                    },
                    Err(e) => eprintln!("{}", e),
                }
            }
        }).unwrap();
    }

    fn input(output_t: Sender<Output>) {
        
        thread::Builder::new().name("input".to_string()).spawn(move|| {
            loop {
                let input = input("").unwrap();
                // Later handle input.
                output_t.send(Output::FromUserInput(format!("input: {:?}", input))).unwrap();
            }
        }).unwrap();
    }

    fn check_maximum_active_connections(max: u16, can_start: Receiver<bool>, allowance: Sender<bool>, finished: Receiver<bool>) {

        let mut number_of_active_connections = 0;
        thread::Builder::new().name("check_maximum_active_connections".to_string()).spawn(move || {
            loop {
                match can_start.try_recv() {
                    Ok(_) => {
                        if number_of_active_connections < max {
                            if let Err(e) = allowance.send(true) {
                                eprintln!("allowance send: {}", e);
                            }
                        }
                    },
                    Err(_) => {},
                }

                match finished.try_recv() {
                    Ok(_) => {
                        if number_of_active_connections != 0 {
                            number_of_active_connections -= 1;
                        }
                    },
                    Err(_) => {},
                }
            }
        }).unwrap();
    }

    fn create_listener(&self) -> TcpListener {

        let socket = SocketAddrV4::new(self.ip(), self.config.port);
        match TcpListener::bind(socket) {
            Ok(listener) => return  listener,
            Err(e) => panic!("Failed to create listener.\n{}", &e),
        }
    }

    fn handle_connection(&self, mut stream: TcpStream,
                         can_start: Sender<bool>, allowance: &Receiver<bool>, finished: Sender<bool>, output: Sender<Output>,
                         db_path: &Path) {

        let users = Arc::clone(&self.users);
        let ids = Arc::clone(&self.ids);
        let waiting_messages = Arc::clone(&self.waiting_messages);
        let location = self.config.save_location.clone();
        let db_location = db_path.to_owned();

        loop {
            if let Err(e) = can_start.send(true) {
                output.send(Output::Error(format!("Failed to send request to create a new thread.\n{}", e))).unwrap();
            }

            let can_start_answer = match allowance.recv() {
                Ok(can_start_sent) => can_start_sent,
                Err(e) => {
                    output.send(Output::Error(format!("Failed to receive an answer to request to create a new thread.\n{}", e))).unwrap();
                    false
                },
            };

            if can_start_answer {
                thread::Builder::new().name("connection".to_string()).spawn(move || {

                    let mut db_conn = Connection::open(db_location).unwrap();

                    match Message::receive(&mut stream, Some(location.clone())) {

                        Ok(message) => {                        

                            let metadata: MetaData = message.metadata();
                            let message_kind: MessageKind = metadata.message_kind();
                            let mut location = metadata.get_message_location(&location);
                            location.push("message.ron");
                            message.save(&location);

                            match message_kind {
                                MessageKind::Text | MessageKind::File => {
                                    let _ = insert_message_into_database(message, &mut db_conn);    
                                },
                                MessageKind::Request => {
                                    Self::receive_request(message, stream, &mut db_conn, output);
                                },
                                _ => {}
                            }
                        },
                        Err(e) => {
                            let err_content = format!(indoc!{
                                "
                                Failed to receive a Message,
                                in Server::handle_connection().
                                error:
                                {}
                                "
                            }, e);
                            output.send(Output::Error(err_content)).unwrap();
                        },
                    }
    
                    if let Err(e) = finished.send(true) {
                        eprintln!("{}", e);
                    }
                }).unwrap(); // !!!!!

                break;
            }
        }
        
    }

    fn receive_request(message: ImplementedMessage,
                       stream: TcpStream, 
                       db_conn: &mut Connection, 
                       output: Sender<Output>) {  

        let metadata = message.metadata();
        let _end_data = message.end_data();

        let content = message.content_move();

        let author = UserLite::new(metadata.author_id(),
                                           metadata.author_username());

        let request = Request::from_ron(&String::from_buff(&content.into_buff())
                                        .unwrap())
                                        .unwrap();

        match request {
            Request::Register(user_unchecked) => {
                Self::user_register(stream, db_conn, user_unchecked, output);
            },
            Request::Login(user_unchecked) => {
                Self::user_login(stream, db_conn, user_unchecked, output);
            },
            Request::GetWaitingMessagesAuto => {
                Self::return_waiting_messages(stream, db_conn, author, output);
            },
            Request::Unknown => todo!(),
        }
    }

    fn user_register(mut stream: TcpStream,
                     db_conn: &mut Connection,
                     user_unchecked: UserUnchecked,
                     output: Sender<Output>) {

        let UserUnchecked {username, password} = user_unchecked;

        // Checks if that username already exists.
        match get_user_id_from_username(db_conn, &username) {
            Ok(id) => {
                let server_reply = ServerReplyRaw::Error(
                    "This username already exists.".to_string(),
                    UserLite::default_user(),
                );
                let message = server_reply.into_message().unwrap();
                message.send(&mut stream).unwrap();
            },
            Err(_) => {
                let id = get_available_id(db_conn);
                let user = User::new(id as u32, username, Password::new(password));

                insert_new_user(db_conn, &user);

                output.send(Output::FromRun(user.clone().to_ron_pretty(None).unwrap())).unwrap();
                let user_lite = UserLite::from_user(&user);

                let server_reply = ServerReplyRaw::User(user_lite, UserLite::default_user());
                let message = server_reply.into_message().unwrap();
                message.send(&mut stream).unwrap();
            },
        }
    }

    fn user_login(mut stream: TcpStream,
                  db_conn: &mut Connection,
                  user_unchecked: UserUnchecked,
                  output: Sender<Output>) -> Result<(), ()> {

        let UserUnchecked {username, password} = user_unchecked;
        let provided_password = password;
                    
        match get_user_id_from_username(db_conn, &username) {
            Ok(id) => {
                let correct_password = match get_user_password(db_conn, id) {
                    Ok(password) => password,
                    Err(_) => {
                        let server_reply = ServerReplyRaw::Error(
                            "Username does not exist.".to_string(),
                             UserLite::default_user(),
                        );
                        let message = server_reply.into_message().unwrap();
                        message.send(&mut stream).unwrap();
                        return Err(())
                    },
                };

                let correct_password = Password::from_hash(correct_password);

                if correct_password.verify(provided_password) {
                    let user_lite = UserLite::new(id as u32, username);
                    let server_reply = ServerReplyRaw::User(user_lite, UserLite::default_user());
                    let message = server_reply.into_message().unwrap();
                    message.send(&mut stream).unwrap();
                } else {
                    let server_reply = ServerReplyRaw::Error(
                        "Incorrect password.".to_string(),
                         UserLite::default_user(),
                    );
                    let message = server_reply.into_message().unwrap();
                    message.send(&mut stream).unwrap();
                }                
            },
            Err(_) => {
                let default_user = UserLite::default_user();
                let server_reply = ServerReplyRaw::Error(
                    format!("User with username: {} does not exist", username),
                    UserLite::default_user(),
                );
                let message = server_reply.into_message().unwrap();
                message.send(&mut stream).unwrap();
            },
        }

        Ok(())
    }

    fn return_waiting_messages(mut stream: TcpStream,
                               db_conn: &mut Connection, 
                               author: UserLite,
                               output: Sender<Output>) {

        let messages = match get_waiting_messages_ids(db_conn, author.id() as usize) {
            Ok(messages) => messages,
            Err(_) => Vec::new(),
        };

        for message_id in messages {
            let message = get_message(db_conn, message_id).unwrap();
            message.send(&mut stream).unwrap();

            delete_waiting_message(db_conn, author.id() as usize).unwrap();
        }
    }

    pub fn ip(&self) -> Ipv4Addr {
        match Ipv4Addr::from_str(&self.config.ip) {
            Ok(ip) => return ip,
            Err(_) => panic!("Failed to get an ip address.\nFailed to parse string from config to Ipv4Addr."),
        }
    }
}

fn get_new_message_id(db_conn: &mut Connection) -> usize {

    let mut stmt = db_conn.prepare("SELECT MAX(id) FROM messages LIMIT 1").unwrap();
    let mut id = stmt.query_map([], |row| {
        let id: usize = match row.get(0) {
            Ok(id) => id,
            Err(_) => 0,
        };
        Ok(id)
    }).unwrap().next().unwrap().unwrap();
    id += 1;

    id
}

fn get_user_id_from_username(db_conn: &mut Connection,
                             username: &str) -> Result<usize, ()> {

    let mut stmt = db_conn.prepare("SELECT id FROM users WHERE username=?1 LIMIT 1").unwrap();
    let mut id_iter = stmt.query_map([username], |row| {
        let id: usize = match row.get(0) {
            Ok(id) => id,
            Err(_) => 0,
        };
        Ok(id)
    }).unwrap();

    match id_iter.next() {
        Some(id) => return  Ok(id.unwrap()),
        None => return Err(()),
    }
}

fn get_available_id(db_conn: &mut Connection) -> usize {

    let mut stmt = db_conn.prepare("SELECT id, last FROM available_ids LIMIT 1").unwrap();
    let mut id = stmt.query_map([], |row| {
        let id: usize = row.get(0).unwrap();
        let last: usize = row.get(1).unwrap();

        if last == 0 {
            db_conn.execute("DELETE FROM available_ids WHERE id=?1", [id]).unwrap();
        } else {
            db_conn.execute("UPDATE available_ids
                                 SET id = id + 1
                                 WHERE last > 1", []).unwrap();
        }

        Ok(id)
    }).unwrap().next().unwrap().unwrap();

    id
}

fn get_user_password(db_conn: &mut Connection, user_id: usize) -> Result<String, ()> {

    let mut stmt = db_conn.prepare("SELECT password FROM users WHERE id=?1").unwrap();

    let mut password_iter = stmt.query_map([user_id], |row| {
        let password: String = row.get(0).unwrap();

        Ok(password)

    }).unwrap();

    match password_iter.next() {
        Some(password) => return  Ok(password.unwrap()),
        None => return Err(()),
    }
}

fn get_waiting_messages_ids(db_conn: &mut Connection,
                            user_id: usize) -> Result<Vec<usize>, ()> {

    let mut stmt = db_conn.prepare("SELECT message_id
                                             FROM waiting_messages
                                             WHERE recipient_id=?1").unwrap();

    let messages_ids_iter = stmt.query_map([user_id], |row| {
        let message_id: usize = row.get(0).unwrap(); 

        Ok(message_id)
    }).unwrap();

    let messages_ids: Vec<usize> = messages_ids_iter.map(|id| id.unwrap()).collect();
    
    if messages_ids.is_empty() {
        Err(())
    } else {
        Ok(messages_ids)
    }
}

fn get_message(db_conn: &mut Connection, message_id: usize) -> Result<ImplementedMessage, ()> {

    let recipients = get_message_recipients_ids(db_conn, message_id).unwrap();
    let recipients: Vec<String> = recipients.iter()
                                            .map(|recip| format!("{}", recip))
                                            .collect();


    let mut stmt = db_conn.prepare("SELECT *
                                                 FROM messages
                                                 WHERE id=?1").unwrap();

    let mut message_iter = stmt.query_map([message_id], |row| {

        let mut message = ImplementedMessage::new();

        let kind: String = row.get(1).unwrap();
        let kind = MessageKind::from_ron(&kind).unwrap();

        let datetime: String = row.get(3).unwrap();
        let datetime = DateTime::parse_from_rfc3339(&datetime).unwrap();
        let datetime = datetime.with_timezone(&Utc);

        let file_name = match row.get_ref_unwrap(7) {
            ValueRef::Null => None,
            ValueRef::Text(path) => {
                let path_string = String::from_buff(path).unwrap();
                Some(path_string)
            },
            _ => panic!()
        };

        let metadata = MetaData::from_data(
            kind,
            row.get(2).unwrap(),
            datetime.into_bytes(),
            row.get(4).unwrap(),
            row.get(5).unwrap(),
            row.get(6).unwrap(),
            recipients.clone(),
            file_name,
        );
        
        let content = row.get(8).unwrap();
        let content = Content::with_data(content);

        let end_data: String = row.get(9).unwrap();
        let end_data = Packet::new(PacketKind::End, Bytes::from_vec(end_data.into_bytes()));

        message.set_metadata(metadata);
        message.set_content(content);
        message.set_end_data(end_data);

        Ok(message)

    }).unwrap();

    match message_iter.next() {
        Some(message) => return  Ok(message.unwrap()),
        None => return Err(()),
    }
}

fn get_message_recipients_ids(db_conn: &mut Connection, message_id: usize) -> Result<Vec<usize>, ()> {

    let mut stmt = db_conn.prepare("SELECT recipient_id
                                                 FROM message_recipients
                                                 WHERE message_id=?1").unwrap();    
    let recipients_ids_iter = stmt.query_map([message_id], |row| {
        let recipient: usize = row.get(0).unwrap();
        Ok(recipient)
    }).unwrap();

    let recipients: Vec<usize> = recipients_ids_iter.map(
                                                |recipient| recipient.unwrap()
                                            )
                                            .collect();
    
    if recipients.is_empty() {
        Err(())
    } else {
        Ok(recipients)
    }
}

fn insert_new_user(db_conn: &mut Connection, user: &User) {

    let id = user.id();
    let id = id.to_sql().unwrap();

    let username = user.username();
    let username = username.to_sql().unwrap();

    let password = user.password().get();
    let password = password.to_sql().unwrap();

    let registration_date = "later".to_sql().unwrap();

    let auth_token = user.auth_token();
    let auth_token = auth_token.to_sql().unwrap();

    db_conn.execute("INSERT INTO users
                         (id, username, password, registration_date, auth_token)
                         VALUES (?1, ?2, ?3, ?4, ?5)", 
                        [
                            id,
                            username,
                            password,
                            registration_date,
                            auth_token,
                        ]).unwrap();
}


fn insert_message_into_database(message: ImplementedMessage, db_conn: &mut Connection) -> Vec<String> {

    let metadata = message.metadata_ref();

    let id = get_new_message_id(db_conn);
    let id = id.to_sql().unwrap();

    let kind = metadata.message_kind().to_ron().unwrap();
    let kind = kind.to_sql().unwrap();
    
    let length = metadata.message_length();
    let length = length.to_sql().unwrap();

    let datetime = metadata.datetime().unwrap().to_rfc3339();
    let datetime = datetime.to_sql().unwrap();

    let author_id = metadata.author_id();
    let author_id = author_id.to_sql().unwrap();

    let author_username = metadata.author_username();
    let author_username = author_username.to_sql().unwrap();

    let recipient_id = metadata.recipient_id();
    let recipient_id = recipient_id.to_sql().unwrap();

    let file_name = metadata.file_name();
    let file_name = file_name.to_sql().unwrap();

    let content = message.content().into_string();
    let content = content.to_sql().unwrap();

    // Change this later so it can accommodate also non string data.
    let end_data = message.end_data().content_move().to_string();
    let end_data = end_data.to_sql().unwrap();

    db_conn.execute("INSERT INTO messages
                            (id, kind, length, datetime, author_id, author_username,
                            recipient_id, file_name, content, end_data)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                            [
                                id.clone(),
                                kind,
                                length,
                                datetime,
                                author_id,
                                author_username,
                                recipient_id,
                                file_name,
                                content,
                                end_data,
                            ]).unwrap();

    let mut non_existent_recipients = Vec::new();
    
    for recipient in metadata.recipients() {
        match get_user_id_from_username(db_conn, &recipient) {
            Ok(recipient_id) => {

                let recipient_id = recipient_id.to_sql().unwrap();

                db_conn.execute("INSERT INTO message_recipients
                                (message_id, recipient_id)
                                VALUES (?1, ?2)",
                                [
                                    id.clone(),
                                    recipient_id.clone()
                                ]).unwrap();
                db_conn.execute("INSERT INTO waiting_messages
                                (message_id, recipient_id)
                                VALUES (?1, ?2)",
                                [
                                    id.clone(),
                                    recipient_id
                                ]).unwrap();
            },
            Err(_) => {
                non_existent_recipients.push(recipient);
            },
        }
    }

    non_existent_recipients
}

fn delete_waiting_message(db_conn: &mut Connection, recipient_id: usize) -> Result<(), ()> {

    db_conn.execute("DELETE FROM waiting_messages
                         WHERE recipient_id=?1", [recipient_id]).unwrap();
    
    Ok(())
}