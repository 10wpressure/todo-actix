use super::utils::return_state;
use actix_web::{HttpRequest, HttpResponse, web};
use crate::json_serialization::to_do_item::ToDoItem;
use crate::schema::to_do;
use crate::diesel;
use diesel::prelude::*;
use crate::auth::jwt::JwtToken;
use crate::database::establish_connection;

pub async fn edit(to_do_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {
    let connection = &mut establish_connection();

    let title = &to_do_item.title;
    let token = JwtToken::decode_from_request(req).unwrap();

    let results = to_do::table
        .filter(to_do::columns::title.eq(&title))
        .filter(to_do::columns::user_id.eq(&token.user_id));

    diesel::update(results)
        .set(to_do::columns::status.eq("done"))
        .execute(connection)
        .expect("Unable to update status");

    HttpResponse::Ok().json(return_state(&token.user_id))
}