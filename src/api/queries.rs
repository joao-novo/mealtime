use rusqlite::{params, Error};
use uuid::{uuid, Uuid};

use crate::structs::{Database, Item, ItemHashEntry};
use std::collections::HashMap;

impl Database {
    fn add_item(&self, item: &Item) {
        match self.connection.execute(
            "INSERT INTO MENU (NAME, PRICE, TIME_TO_PREPARE) values (?1, ?2, ?3)",
            params![item.name, item.price, item.time_to_prepare],
        ) {
            Ok(_) => println!("Item {} added to menu.", item.name),
            Err(e) => panic!("{e}: Could not add item to menu."),
        }
    }

    fn remove_item(&self, name: String) {
        match self
            .connection
            .execute("DELETE FROM MENU WHERE NAME = ?1", params![name])
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
                id: row.get(0)?,
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
                id: row.get(0)?,
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

    fn update_item(&self, old_name: String, new_item: &Item) {
        match self.connection.execute(
            "UPDATE MENU SET NAME = ?1, PRICE = ?2, TIME_TO_PREPARE = ?3 WHERE NAME = ?4;",
            params![
                new_item.name,
                new_item.price,
                new_item.time_to_prepare,
                old_name
            ],
        ) {
            Ok(_) => println!("Updated item successfully."),
            Err(e) => panic!("{e}: Could not update item."),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::{Database, Item};
    use rusqlite::Connection;
    use uuid::Uuid;

    fn setup_db() -> Database {
        // Opens an in-memory database for testing to prevent parallelism issues
        let conn = Connection::open_in_memory();
        let db = Database {
            connection: conn.unwrap(),
        };

        // Creates an equivalent table to the original one
        db.connection
            .execute(
                "CREATE TABLE MENU (
                ID UUID PRIMARY KEY,
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
            id: Uuid::new_v4().to_string(),
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
            id: Uuid::new_v4().to_string(),
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
            id: Uuid::new_v4().to_string(),
            name: String::from("Burger"),
            price: 4.99,
            time_to_prepare: 5,
        });
        db.add_item(&Item {
            id: Uuid::new_v4().to_string(),
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
            id: Uuid::new_v4().to_string(),
            name: String::from("Burger"),
            price: 4.99,
            time_to_prepare: 5,
        });
        let item = db.get_item_by_name(String::from("Burger"));
        assert!(item.name == String::from("Burger"));
    }

    // #[test]
    // fn test_update_item() {
    //     let db = setup_db();

    //     db.add_item(&Item {
    //         name: String::from("Burger"),
    //         price: 4.99,
    //         time_to_prepare: 5,
    //     });

    //     // db.update_item(
    //     //     String::from("Burger"),
    //     //     &Item {
    //     //         name: String::from("Cheeseburger"),
    //     //         price: 5.99,
    //     //         time_to_prepare: 6,
    //     //     },
    //     // );
    //     let item = db.get_item_by_name(String::from("Cheeseburger"));
    //     assert!(item.price == 5.99 && item.time_to_prepare == 6);
    // }
}
