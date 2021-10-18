use std::{path::{Path, PathBuf}, sync::mpsc::Sender};

use chrono::{DateTime, Utc};
use nardol::{error::NetCommsError, prelude::{Bytes, FromBytes, FromRon, IntoBytes, Packet, PacketKind, ToRon}};
use rusqlite::{Connection, Row, ToSql, types::ValueRef};
use shared::{Content, ImplementedMessage, MessageKind, MetaData};

use super::Output;



pub fn open_database(db_path: &Path, _output_t: Sender<Output>) -> Result<(), NetCommsError> {
        
    let db_conn =  Connection::open(db_path).unwrap();

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
        "CREATE TABLE users (
            id                  INTEGER NOT NULL,
            username            TEXT NOT NULL
        )", []) {
        // Falls here if table already exist, check if table has correct structure is necessary.
    };    

    Ok(())
}

pub fn insert_message(db_conn: &mut Connection, message: ImplementedMessage) {

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
            },
            Err(_) => {
                non_existent_recipients.push(recipient);
            },
        }
    }
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

pub fn get_message_by_id(db_conn: &mut Connection, message_id: usize) -> Result<ImplementedMessage, ()> {

    let recipients = get_message_recipients(db_conn, message_id).unwrap();
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

// Not finished.
pub fn get_message_by_author(db_location: PathBuf,
                             author_username: String,
                             limit: Option<u64>) -> Result<Vec<ImplementedMessage>, ()> {

    let db_conn = Connection::open(&db_location).unwrap();

    let mut stmt = db_conn.prepare("SELECT *
                                    FROM messages
                                    WHERE author_username=?1
                                    LIMIT ?2").unwrap(); 

    let params = match limit {
        Some(limit) => {
            [author_username, limit.to_string()]
        },
        None => [author_username, String::from("1")],
    };  

    // I believe that sql can take integer argument as a string, but I may be wrong in which case this is gonna be a lot of work.
    let mut messages_iter = stmt.query_map(params, |row| {

        let message_id = row.get(0).unwrap();

        let mut db_conn_recipients = Connection::open(db_location.clone()).unwrap();

        let recipients = get_message_recipients(&mut db_conn_recipients, message_id).unwrap();

        let message = create_message_from_row(row, recipients);
        
        Ok(message)
        
    }).unwrap();

    let mut messages = Vec::new();

    for message in messages_iter.into_iter() {
        messages.push(message.unwrap())
    }

    Ok(messages)
}

pub fn get_message_by_time(db_conn: &mut Connection, datetime: DateTime<Utc>) -> Result<ImplementedMessage, ()> {
    Err(())
}

pub fn get_message_recipients(db_conn: &mut Connection, message_id: usize) -> Result<Vec<String>, ()> {
    Err(())
}


pub fn create_message_from_row(row: &rusqlite::Row<'_>, recipients: Vec<String>) -> ImplementedMessage {

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

    message
}