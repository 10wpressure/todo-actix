use actix_web::HttpRequest;
use actix_web::Responder;
use crate::diesel::prelude::*;
use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do::dsl::{to_do, title};
use crate::schema::to_do::id;
use crate::views::to_do::utils::return_state;


pub async fn create(req: HttpRequest) -> impl Responder {
    let req_title = req.match_info().get("title").unwrap().to_string();
    let title_ref = req_title.clone();
    let mut connection = establish_connection();
    let items = to_do
        .filter(title.eq(req_title))
        .order(id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
    if items.is_empty() {
        let new_post = NewItem::new(title_ref);
        let _ = diesel::insert_into(to_do).values(&new_post).execute(&mut connection);
    }
    return_state()
}