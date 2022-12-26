use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpRequest;
use crate::state::read_file;
use crate::to_do::to_do_factory;
use crate::processes::process_input;

pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file("./state.json");
    let title: String = req.match_info().get("title").unwrap().to_string();
    let title_reference: String = title.clone();
    let item = to_do_factory(&String::from("pending"), &title).expect("create");
    process_input(item, "create".to_string(), &state);
    format!("Created {}", title_reference)
}