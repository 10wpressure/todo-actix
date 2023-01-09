use super::utils::return_state;
use actix_web::{HttpResponse, web};
use crate::json_serialization::to_do_item::ToDoItem;
use crate::schema::to_do;
use crate::diesel;
use diesel::prelude::*;
use crate::database::establish_connection;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let connection = &mut establish_connection();

    let results = to_do::table
        .filter(to_do::columns::title
            .eq(&to_do_item.title));
    diesel::update(results)
        .set(to_do::columns::status
            .eq("done"))
        .execute(connection)
        .expect("Unable to update status");

    HttpResponse::Ok().json(return_state())
}