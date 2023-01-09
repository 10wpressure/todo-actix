use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};
use crate::database::establish_connection;
use crate::json_serialization::new_user::NewUserSchema;
use crate::models::user::new_user::NewUser;
use crate::schema::users;

pub async fn create(new_user: web::Json<NewUserSchema>) -> HttpResponse {
    let connection = &mut establish_connection();
    let new_user = NewUser::new(
        new_user.name.to_string(),
        new_user.email.to_string(),
        new_user.password.to_string(),
    );
    let insert_result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection);

    match insert_result {
        Ok(_) => HttpResponse::Created().await.unwrap(),
        Err(_) => HttpResponse::BadRequest().await.unwrap(),
    }
}