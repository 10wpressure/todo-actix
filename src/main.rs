extern crate core;
extern crate dotenv;
#[macro_use] extern crate diesel;

use actix_web::{App, HttpServer};
use actix_service::Service;

mod views;
mod to_do;
mod state;
mod processes;
mod json_serialization;
mod database;
mod schema;
mod models;
mod auth;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .wrap_fn(|req, srv| {
            if req.path().contains("/item/") {
                match views::token::process_token(&req) {
                    Ok(token) => {
                        println!("the token is passable: {:?}", token)
                    }
                    Err(message) => println!("token error: {}", message)
                }
            }
            let fut = srv.call(req);
            async {
                let result = fut.await?;
                Ok(result)
            }
        })
        .configure(views::views_factory))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}