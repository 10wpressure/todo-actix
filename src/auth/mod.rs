use crate::auth::processes::{check_password, extract_header_token};
use actix_web::dev::ServiceRequest;

pub mod jwt;
mod processes;

pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match extract_header_token(request) {
        Ok(token) => match check_password(token) {
            Ok(token) => Ok(token),
            Err(message) => Err(message),
        },
        Err(message) => Err(message),
    }
}

#[cfg(test)]
mod process_token_tests {
    use actix_web::test;
    use crate::auth::jwt::JwtToken;
    use super::*;

    #[test]
    async fn no_token_process_token() {
        let req = test::TestRequest::default()
            .append_header(("test", "test"))
            .to_srv_request();

        match process_token(&req) {
            Err(message) => assert_eq!("user-token header is missing", message),
            _ => panic!("No token in request header should fail")
        }
    }

    #[test]
    async fn incorrect_token() {
        let req = test::TestRequest::default()
            .append_header(("user-token", "test"))
            .to_srv_request();

        match process_token(&req) {
            Err(message) => assert_eq!("Could not decode", message),
            _ => panic!("No token in request header should fail")
        }
    }

    #[test]
    async fn correct_token() {
        let encoded_token: String = JwtToken::encode(32);
        let req = test::TestRequest::default()
            .append_header(("user-token", encoded_token))
            .to_srv_request();
        match process_token(&req) {
            Ok(token) => assert_eq!("passed", token),
            _ => panic!("encoded token should pass")
        }
    }
}