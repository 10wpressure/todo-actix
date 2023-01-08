use actix_web::{HttpResponse, web};
use serde_json::{Map,Value};
use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;
use crate::state::{FILE_NAME, read_file};
use crate::to_do::to_do_factory;
use crate::views::to_do::utils::return_state;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file(FILE_NAME);
    let title_reference: &String = &to_do_item.title.clone();
    let title: String = to_do_item.title.clone();

    let status: String = match state.get(title_reference) {
        None => return HttpResponse::NotFound().json(format!("{} not in state", title_reference)),
        Some(result) => result.to_string().replace('\"', "")
    };

    if status == to_do_item.status {
        return HttpResponse::Ok().json(return_state());
    }

    match to_do_factory(&status, &title) {
        Ok(item) => process_input(item, String::from("edit"), &state),
        Err(_) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
    }
    HttpResponse::Ok().json(return_state())
}