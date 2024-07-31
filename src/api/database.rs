use rusqlite::Connection;

pub fn handle_database_connection() -> Connection {
    match Connection::open("db/menu.db") {
        Ok(conn) => {
            println!("Connection established");
            return conn;
        }
        Err(_) => panic!("Could not connect to database"),
    }
}
