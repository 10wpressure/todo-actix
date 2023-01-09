use actix_web::dev::ServiceRequest;
use crate::auth::processes::{check_password, extract_header_token};

pub mod jwt;
mod processes;

pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match extract_header_token(request) {
        Ok(token) => match check_password(token) {
            Ok(token) => Ok(token),
            Err(message) => Err(message)
        },
        Err(message) => Err(message),
    }
}