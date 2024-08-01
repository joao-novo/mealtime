use crate::structs::Database;
use rusqlite::Connection;

pub fn handle_database_connection() -> Database {
    match Connection::open("./db/menu.db") {
        Ok(conn) => {
            println!("Connection established.");
            return Database { connection: conn };
        }
        Err(_) => panic!("Could not connect to database."),
    }
}
