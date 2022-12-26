use actix_web::{Responder};
use crate::views::to_do::utils::return_state;

pub async fn get() -> impl Responder {
    return_state()
}