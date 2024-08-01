use crate::structs::{Database, Item};

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

    fn get_items(&self) -> Vec<Item> {
        let mut stmt = match self.connection.prepare("SELECT * FROM MENU ITEM;") {
            Ok(stmt) => stmt,
            Err(e) => panic!("{e}: Could not retrieve items."),
        };

        let items = match stmt.query_map([], |row| {
            Ok(Item {
                name: row.get(1)?,
                price: row.get(2)?,
                time_to_prepare: row.get(3)?,
            })
        }) {
            Ok(item) => item,
            Err(e) => panic!("{e}"),
        };

        let mut result = Vec::new();

        for item in items {
            result.push(item.unwrap());
        }
        result
    }

    fn get_item_by_name(&self, name: String) -> Item {
        todo!();
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
        assert_eq!(String::from("Salad"), items.last().unwrap().name);
    }

    #[test]
    fn test_get_items() {
        let db = handle_database_connection();
        let items = db.get_items();
        let mut names = Vec::new();
        for item in items {
            names.push(item.name);
        }
        assert_eq!(
            names,
            vec![String::from("Burger"), String::from("Small Fries"),]
        );
    }
}
