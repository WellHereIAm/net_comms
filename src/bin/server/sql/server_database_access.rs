use std::{path::Path, sync::mpsc::Sender};

use chrono::{DateTime, Utc};
use nardol::{error::NetCommsError, prelude::{Bytes, FromBytes, FromRon, IntoBytes, Packet, PacketKind, ToRon}};
use rusqlite::{Connection, ToSql, types::ValueRef};
use shared::{Content, ImplementedMessage, MessageKind, MetaData, user::User};

use crate::server::Output;


pub fn open_database(db_path: &Path, _output_t: Sender<Output>) -> Result<(), NetCommsError> {

        
    let db_conn =  Connection::open(db_path).unwrap();

    if let Err(_) = db_conn.execute(
        "CREATE TABLE users (
            id                  INTEGER NOT NULL,
            username            TEXT NOT NULL,
            password            TEXT NOT NULL,
            registration_date   TEXT NOT NULL,
            auth_token          TEXT DEFAULT NULL
        )", []) {
        // Falls here if table already exist, check if table has correct structure is necessary.
    }

    // id should be later changed to AUTO INCREMENT
    if let Err(_) = db_conn.execute(
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
    )", []) {
        // Falls here if table already exist, check if table has correct structure is necessary.
    };

    if let Err(_) = db_conn.execute(
        "CREATE TABLE message_recipients (
            message_id          INTEGER NOT NULL,
            recipient_id        INTEGER NOT NULL
    )", []) {
        // Falls here if table already exist, check if table has correct structure is necessary.
    };

    if let Err(_) = db_conn.execute(
        "CREATE TABLE waiting_messages (
            message_id          INTEGER NOT NULL,
            recipient_id        INTEGER NOT NULL
        )", []) {
        // Falls here if table already exist, check if table has correct structure is necessary.
    };

    // Last available id using integer as bool.
    if let Err(_) = db_conn.execute(
        "CREATE TABLE available_ids (
            id                  INTEGER NOT NULL,
            last                INTEGER NOT NULL
    )", []) {
        // Falls here if table already exist, check if table has correct structure is necessary.
    };

    let first_id = 2;

    db_conn.execute("INSERT INTO available_ids (id, last) VALUES (?1, ?2)", [first_id, 1]).unwrap();

    Ok(())
}


pub fn get_new_message_id(db_conn: &mut Connection) -> usize {

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

pub fn get_user_id_from_username(db_conn: &mut Connection,
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

pub fn get_available_id(db_conn: &mut Connection) -> usize {

    let mut stmt = db_conn.prepare("SELECT id, last FROM available_ids LIMIT 1").unwrap();
    let mut id = stmt.query_map([], |row| {
    let id: usize = row.get(0).unwrap();
    let last: usize = row.get(1).unwrap();

    if last == 0 {
        println!("Is not last.");
        db_conn.execute("DELETE FROM available_ids WHERE id=?1", [id]).unwrap();
    } else {
        println!("Is last.");
        db_conn.execute("UPDATE available_ids
                             SET id = id + 1
                             WHERE last > 0", []).unwrap();
    }
        Ok(id)
    }).unwrap().next().unwrap().unwrap();

    id
}

pub fn get_user_password(db_conn: &mut Connection, user_id: usize) -> Result<String, ()> {

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

pub fn get_waiting_messages_ids(db_conn: &mut Connection,
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

pub fn get_message(db_conn: &mut Connection, message_id: usize) -> Result<ImplementedMessage, ()> {

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

pub fn get_message_recipients_ids(db_conn: &mut Connection, message_id: usize) -> Result<Vec<usize>, ()> {

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

pub fn insert_new_user(db_conn: &mut Connection, user: &User) {

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

pub fn insert_message_into_database(message: ImplementedMessage, db_conn: &mut Connection) -> Vec<String> {

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

pub fn delete_waiting_message(db_conn: &mut Connection, recipient_id: usize) -> Result<(), ()> {

    db_conn.execute("DELETE FROM waiting_messages
                         WHERE recipient_id=?1", [recipient_id]).unwrap();
    
    Ok(())
}