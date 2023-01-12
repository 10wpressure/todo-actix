use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::models::item::generic_item::Item;
use crate::schema::to_do::dsl::*;
use diesel::prelude::*;

use crate::to_do::to_do_factory;

pub fn return_state(current_user_id: &i32) -> ToDoItems {
    let connection = &mut establish_connection();
    let items = to_do
        .order(id.asc())
        .filter(user_id.eq(current_user_id))
        .load::<Item>(connection)
        .expect("Error loading items");
    let mut array_buffer = Vec::new();
    for item in items {
        let item = to_do_factory(&item.status, &item.title).unwrap();
        array_buffer.push(item);
    }
    ToDoItems::new(array_buffer)
}
