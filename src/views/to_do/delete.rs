use crate::auth::jwt::JwtToken;
use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::models::item::generic_item::Item;
use crate::schema::to_do;
use crate::views::to_do::utils::return_state;
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::prelude::*;

pub async fn delete(to_do_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {
    let connection = &mut establish_connection();

    let title = &to_do_item.title;
    let token = JwtToken::decode_from_request(req).unwrap();

    let items = to_do::table
        .filter(to_do::columns::title.eq(title))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(connection)
        .unwrap();

    diesel::delete(&items[0])
        .execute(connection)
        .expect("Unable to update status");

    HttpResponse::Ok().json(return_state(&token.user_id))
}
