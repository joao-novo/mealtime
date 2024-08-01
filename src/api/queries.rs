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

    fn get_items(&self) {
        todo!();
    }

    fn get_item_by_name(&self, name: String) {
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
