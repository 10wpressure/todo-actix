use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::StatusCode;
use serde::Serialize;
use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> Self {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();
        for item in input_items {
            match item {
                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct),
                ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),
                _ => eprintln!("unknown item type"),
            }
        }
        let done_count: i8 = done_array_buffer.len() as i8;
        let pending_count: i8 = pending_array_buffer.len() as i8;

        Self {
            pending_items: pending_array_buffer,
            done_items: done_array_buffer,
            pending_item_count: pending_count,
            done_item_count: done_count,
        }
    }
}

impl Responder for ToDoItems {
    type Body = String;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<String> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::new(StatusCode::OK).set_body(body)
    }
}