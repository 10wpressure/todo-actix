use actix_web::{HttpResponse, web};
use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::schema::to_do;
use crate::views::to_do::utils::return_state;
use crate::diesel;
use diesel::prelude::*;
use crate::models::item::item::Item;

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let connection = &mut establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title
            .eq(&to_do_item.title))
        .order(to_do::columns::id.asc())
        .load::<Item>(connection)
        .unwrap();

    diesel::delete(&items[0])
        .execute(connection)
        .expect("Unable to update status");

    HttpResponse::Ok().json(return_state())
}