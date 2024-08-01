use rusqlite::Connection;

pub struct Database {
    pub connection: Connection,
}

pub struct Item {
    pub name: String,
    pub price: f32,
    pub time_to_prepare: u32,
}
