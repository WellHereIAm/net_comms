use std::{sync::{Arc, Mutex, mpsc::{self, Sender}}, thread};

use rusqlite::{params, Connection};

#[test]
fn db_test() {

    use rusqlite::{params, Connection};

    let conn = Connection::open("D:\\stepa\\Documents\\Rust\\net_comms_logs\\server_logs").unwrap();
    conn.execute(
        "CREATE TABLE users (
            id          INTEGER PRIMARY KEY NOT NULL,
            username    TEXT NOT NULL,
            password    TEXT NOT NULL,
            auth_token  TEXT DEFAULT NULL

        )", []).unwrap();

    let conn = Arc::new(Mutex::new(conn));


    let (db_t, db_r) = mpsc::channel();

    let _ = thread::spawn(move || {
        let (thread_t, thread_r) = mpsc::channel::<String>();

        db_t.send(thread_t).unwrap()
    });
}