use crate::api::database::{handle_database_connection, Database};
use std::io;

pub mod api;
fn handle_commands(db: &Database, command: String) {
    todo!();
}

fn main() {
    let db = handle_database_connection();
    loop {
        let mut command = String::new();
        match io::stdin().read_line(&mut command) {
            Ok(_) => {
                handle_commands(&db, command);
            }
            Err(error) => {
                panic!("Error reading input: {error:?}")
            }
        }
    }
}
