use super::jwt;
use actix_web::dev::ServiceRequest;

pub fn check_password(password: String) -> Result<String, &'static str> {
    match jwt::JwtToken::decode(password) {
        Ok(_token) => Ok(String::from("passed")),
        Err(err) => Err(err),
    }
}

pub fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match request.headers().get("user-token") {
        Some(token) => match token.to_str() {
            Ok(processed_password) => Ok(String::from(processed_password)),
            Err(_processed_password) => Err("user-token is not a string"),
        },
        None => Err("user-token header is missing"),
    }
}

#[cfg(test)]
mod check_credentials_tests {
    use crate::auth::jwt::JwtToken;
    use actix_web::test;
    use super::*;

    #[test]
    async fn check_correct_password() {
        let token = JwtToken::encode(32);
        match check_password(token) {
            Ok(password) => assert_eq!("passed", password),
            _ => panic!("correct password should pass")
        }
    }

    #[test]
    async fn check_incorrect_password() {
        let password: String = String::from("test");

        match check_password(password) {
            Err(message) => assert_eq!("Could not decode", message),
            _ => panic!("check password should not be able to be decoded")
        }
    }

    #[test]
    async fn no_token_in_extract_header_token() {
        let mock_request = test::TestRequest::default()
            .append_header(("test", "test"))
            .to_srv_request();

        match extract_header_token(&mock_request) {
            Err(message) => assert_eq!("user-token header is missing", message),
            _ => panic!("token should not be present in service request")
        }
    }

    #[test]
    async fn correct_token_in_extract_header_token() {
        let mock_request = test::TestRequest::default().append_header(("user-token", "test")).to_srv_request();

        match extract_header_token(&mock_request) {
            Ok(token) => assert_eq!(String::from("test"), token),
            _ => panic!("token should be present in the header")
        }
    }
}