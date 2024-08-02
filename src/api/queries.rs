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
        match self
            .connection
            .execute("DELETE FROM MENU WHERE NAME = ?1", &[&name])
        {
            Ok(_) => println!("Item {name} removed from menu."),
            Err(e) => panic!("{e}: Could not remove item from menu."),
        }
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
    use crate::{
        api::database::handle_database_connection,
        structs::{Database, Item},
    };
    use rusqlite::Connection;

    fn setup_db() -> Database {
        let conn = Connection::open_in_memory();
        let db = Database {
            connection: conn.unwrap(),
        };
        db.connection
            .execute(
                "CREATE TABLE MENU (
                ID INT PRIMARY KEY,
                NAME TEXT NOT NULL,
                PRICE REAL NOT NULL,
                TIME_TO_PREPARE INT NOT NULL
            );",
                [],
            )
            .unwrap();
        db
    }

    #[test]
    fn test_add_item() {
        let db = setup_db();
        db.add_item(&Item {
            name: String::from("Salad"),
            price: 5.99,
            time_to_prepare: 6,
        });
        let items = db.get_items();
        assert!(items["Salad"].price == 5.99 && items["Salad"].time_to_prepare == 6);
    }

    #[test]
    fn test_remove_item() {
        let db = setup_db();
        db.add_item(&Item {
            name: String::from("Burger"),
            price: 4.99,
            time_to_prepare: 5,
        });
        db.remove_item(String::from("Burger"));
        let items = db.get_items();
        assert!(!items.contains_key("Burger"));
    }

    #[test]
    fn test_get_items() {
        let db = setup_db();
        db.add_item(&Item {
            name: String::from("Burger"),
            price: 4.99,
            time_to_prepare: 5,
        });
        db.add_item(&Item {
            name: String::from("Small Fries"),
            price: 3.99,
            time_to_prepare: 4,
        });
        let items = db.get_items();
        assert!(items.contains_key("Burger") && items.contains_key("Small Fries"));
    }

    #[test]
    fn test_get_item_by_name() {
        let db = setup_db();
        db.add_item(&Item {
            name: String::from("Burger"),
            price: 4.99,
            time_to_prepare: 5,
        });
        let item = db.get_item_by_name(String::from("Burger"));
        assert!(item.name == String::from("Burger"));
    }
}
