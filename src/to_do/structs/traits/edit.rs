use serde_json::{json, Map, Value};
use crate::state::write_to_file;

pub trait Edit {
    fn set_to_done(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!("done"));
        write_to_file("./state.json", state);
        println!("\n\n{} is being set to done\n\n", title);
    }
    fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!("pending"));
        write_to_file("./state.json", state);
        println!("\n\n{} is being set to pending\n\n", title);
    }
    fn set_to_on_hold(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!("on_hold"));
        write_to_file("./state.json", state);
        println!("\n\n{} is being set to on hold\n\n", title);
    }
}