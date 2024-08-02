use crate::structs::{Database, Item, ItemHashEntry};
use std::collections::HashMap;

impl Database {
    fn add_item(&self, item: &Item) {
        match self.connection.execute(
            "INSERT INTO MENU (NAME, PRICE, TIME_TO_PREPARE) values (?1, ?2, ?3)",
            &[
                &item.name,
                &item.price.to_string(),
                &item.time_to_prepare.to_string(),
            ],
        ) {
            Ok(_) => println!("Item {} added to menu.", item.name),
            Err(e) => panic!("{e}: Could not add item to menu."),
        }
    }

    fn remove_item(&self, name: String) {
        todo!();
    }

    fn get_items(&self) -> HashMap<String, ItemHashEntry> {
        let mut stmt = match self.connection.prepare("SELECT * FROM MENU;") {
            Ok(stmt) => stmt,
            Err(e) => panic!("{e}: Could not query database."),
        };

        let items = match stmt.query_map([], |row| {
            Ok(Item {
                name: row.get(1)?,
                price: row.get(2)?,
                time_to_prepare: row.get(3)?,
            })
        }) {
            Ok(items) => items,
            Err(e) => panic!("{e}: Failed to retrieve items."),
        };

        let mut result = HashMap::new();

        for item in items.map(|i| i.unwrap()) {
            result.insert(
                item.name,
                ItemHashEntry {
                    price: item.price,
                    time_to_prepare: item.time_to_prepare,
                },
            );
        }
        result
    }

    fn get_item_by_name(&self, name: String) -> Item {
        let mut stmt = match self
            .connection
            .prepare("SELECT * FROM MENU WHERE NAME = ?1;")
        {
            Ok(stmt) => stmt,
            Err(e) => panic!("{e}: Could not query database."),
        };

        let mut rows = match stmt.query_map([name], |row| {
            Ok(Item {
                name: row.get(1)?,
                price: row.get(2)?,
                time_to_prepare: row.get(3)?,
            })
        }) {
            Ok(row) => row,
            Err(e) => panic!("{e}: Could not retrieve item."),
        };
        rows.next().unwrap().unwrap()
    }

    fn update_item(&self, old_name: String, new_item: Item) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::database::handle_database_connection, structs::Item};

    #[test]
    fn test_add_item() {
        let db = handle_database_connection();
        db.add_item(&Item {
            name: String::from("Salad"),
            price: 5.99,
            time_to_prepare: 6,
        });
        let items = db.get_items();
        assert!(items["Salad"].price == 5.99 && items["Salad"].time_to_prepare == 6);
    }

    #[test]
    fn test_get_items() {
        let db = handle_database_connection();
        let items = db.get_items();
        assert!(items.contains_key("Burger") && items.contains_key("Small Fries"));
    }

    #[test]
    fn test_get_item_by_name() {
        let db = handle_database_connection();
        let item = db.get_item_by_name(String::from("Burger"));
        assert!(item.name == String::from("Burger"));
    }
}
