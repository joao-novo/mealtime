use rusqlite::Connection;

pub fn add_item(conn: Connection, name: String, price: f32, time_to_prepare: i32) {
    match conn.execute(
        "INSERT INTO MENU (NAME, PRICE, TIME_TO_PREPARE) values (?1, ?2, ?3)",
        &[&name, &price.to_string(), &time_to_prepare.to_string()],
    ) {
        Ok(_) => println!("Item {name} added to menu"),
        Err(e) => panic!("{e}: Could not add item to menu"),
    }
}
