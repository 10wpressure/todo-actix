
use actix_web::{HttpResponse, web};
use serde_json::{Map, Value};
use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;
use crate::state::{FILE_NAME, read_file};
use crate::to_do::to_do_factory;
use crate::views::to_do::utils::return_state;

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file(FILE_NAME);
    let title: String = to_do_item.title.clone();
    let status: String = to_do_item.status.clone();

    match to_do_factory(&status, &title) {
        Ok(item) => process_input(item, String::from("delete"), &state),
        Err(_) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
    }

    HttpResponse::Ok().json(return_state())
}