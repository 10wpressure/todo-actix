use crate::auth::jwt::JwtToken;
use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::login::Login;
use crate::models::user::generic_user::User;
use crate::schema::users;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn login(credentials: web::Json<Login>) -> HttpResponse {
    let username = &credentials.username;
    let password = &credentials.password;
    let connection = &mut establish_connection();
    let users = users::table
        .filter(users::columns::username.eq(username.as_str()))
        .load::<User>(connection)
        .unwrap();

    if users.is_empty() {
        return HttpResponse::NotFound().await.unwrap();
    } else if users.len() > 1 {
        log::error!("multiple users have the username: {}", credentials.username);
        return HttpResponse::Conflict().await.unwrap();
    };

    match users[0].clone().verify(password.to_string()) {
        true => {
            let token = JwtToken::encode(users[0].clone().id);
            HttpResponse::Ok()
                .append_header(("token", token))
                .await
                .unwrap()
        }
        false => HttpResponse::Unauthorized().await.unwrap(),
    }
}
