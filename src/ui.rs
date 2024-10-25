use crate::structs::Database;
use cursive::{
    views::{Dialog, DummyView, LinearLayout, ListView},
    Cursive,
};
pub fn display_menu(app: &mut Cursive, db: Database) {
    let items = db.get_items();
    app.pop_layer();
    app.add_layer(Dialog::new().title("MealTime/Menu").content(
        ListView::new().child("test", DummyView).with(|menu| {
            for i in items {
                menu.add_child(i.name);
            }
        }),
    ));
}
