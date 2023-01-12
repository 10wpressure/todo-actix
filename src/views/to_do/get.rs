use crate::auth::jwt::JwtToken;
use crate::views::to_do::utils::return_state;
use actix_web::{HttpRequest, Responder};

pub async fn get(req: HttpRequest) -> impl Responder {
    let token = JwtToken::decode_from_request(req).unwrap();
    return_state(&token.user_id)
}
