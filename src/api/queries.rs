use super::database::Database;

impl Database {
    fn add_item(&self, name: String, price: f32, time_to_prepare: i32) {
        match self.connection.execute(
            "INSERT INTO MENU (NAME, PRICE, TIME_TO_PREPARE) values (?1, ?2, ?3)",
            &[&name, &price.to_string(), &time_to_prepare.to_string()],
        ) {
            Ok(_) => println!("Item {name} added to menu."),
            Err(e) => panic!("{e}: Could not add item to menu."),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::database::handle_database_connection;

    #[test]
    fn test_add_item() {
        let db = handle_database_connection();
        db.add_item(String::from("Salad"), 5.99, 6);
        let mut stmt = db
            .connection
            .prepare("SELECT ITEM.NAME from MENU ITEM;")
            .unwrap();
        let items = stmt
            .query_map([], |row| Ok(row.get::<usize, String>(0)?))
            .unwrap();
        assert_eq!(String::from("Salad"), items.last().unwrap().unwrap());
    }
}
