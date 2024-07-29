use std::io;

fn handle_commands(command: String) {}

fn main() {
    // handle_database_connection();
    loop {
        let mut command = String::new();
        match io::stdin().read_line(&mut command) {
            Ok(_) => {
                handle_commands(command);
            }
            Err(error) => {
                panic!("Error reading input: {error:?}")
            }
        }
    }
}
