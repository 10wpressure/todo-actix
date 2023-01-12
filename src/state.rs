use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde_json::{json, Map};
use serde_json::value::Value;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let empty = create_init_state_file(file_name); // TODO: if no json or empty json - init it
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name, new_data.to_string()).expect("Unable to write to file");
}

fn create_init_state_file(file_name: &str) -> bool {
    let is_file = Path::new(file_name).is_file();
    let mut is_empty = false;
    if !is_file {
        File::create(file_name).expect("Could not create a file");
        is_empty = true;
    }
    is_empty
}