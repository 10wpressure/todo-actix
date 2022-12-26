use serde_json::{Map, Value};
use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::{FILE_NAME, read_file};
use crate::to_do::{ItemTypes, to_do_factory};

pub fn return_state() -> ToDoItems {
    let state: Map<String, Value> = read_file(FILE_NAME);
    let mut array_buffer = Vec::new();
    for (key, value) in state.iter() {
        let item_type: String = value.as_str().unwrap().to_string();
        let item: ItemTypes = to_do_factory(&item_type, key).unwrap();
        array_buffer.push(item);
    };
    ToDoItems::new(array_buffer)
}