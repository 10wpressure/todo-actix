extern crate core;
extern crate dotenv;
extern crate diesel;

use actix_web::{App, HttpServer, HttpResponse};
use actix_service::Service;
use futures::future::{ok, Either};
use log;
use env_logger;
use crate::auth::process_token;

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
    env_logger::init();
    HttpServer::new(|| App::new()
        .wrap_fn(|req, srv| {
            let request_url = req.uri().path().to_string();
            let passed: bool;

            if req.path().contains("/item/") {
                match process_token(&req) {
                    Ok(_token) => passed = true,
                    Err(_message) => passed = false,
                }
            } else {
                passed = true;
            }
            let result = match passed {
                true => Either::Left(srv.call(req)),
                false => Either::Right(ok(req.into_response(HttpResponse::Unauthorized().finish()))),
            };

            async move {
                let result = result.await?;
                log::info!("{}->{}", &request_url, &result.status());
                Ok(result)
            }
        })
        .configure(views::views_factory))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}