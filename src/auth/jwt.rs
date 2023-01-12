extern crate hmac;
extern crate jwt;
extern crate sha2;

use actix_web::HttpRequest;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::{Header, Token, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub struct JwtToken {
    pub user_id: i32,
    pub body: String,
}

impl JwtToken {
    pub fn encode(user_id: i32) -> String {
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret").unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id);
        let token: String = claims.sign_with_key(&key).unwrap();
        token
    }

    pub fn decode(encoded_token: String) -> Result<JwtToken, &'static str> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret").unwrap();
        let token_str: &str = encoded_token.as_str();
        let token: Result<Token<Header, BTreeMap<String, i32>, _>, _> =
            VerifyWithKey::verify_with_key(token_str, &key);
        match token {
            Ok(token) => {
                let _header: &Header = token.header();
                let claims = token.claims();
                Ok(JwtToken {
                    user_id: claims["user_id"],
                    body: encoded_token,
                })
            }
            Err(_) => Err("Could not decode"),
        }
    }

    pub fn decode_from_request(req: HttpRequest) -> Result<JwtToken, &'static str> {
        match req.headers().get("user-token") {
            Some(token) => JwtToken::decode(String::from(token.to_str().unwrap())),
            None => Err("No token received"),
        }
    }
}

#[cfg(test)]
mod jwt_tests {
    use actix_web::test;
    use super::*;

    #[test]
    async fn encode_decode() {
        let encoded_token: String = JwtToken::encode(32);
        let decoded_token: JwtToken = JwtToken::decode(encoded_token).unwrap();
        assert_eq!(32, decoded_token.user_id);
    }

    #[test]
    async fn decoded_incorrect_token() {
        let encoded_token: String = "test".to_string();
        match JwtToken::decode(encoded_token) {
            Err(message) => assert_eq!("Could not decode", message),
            _ => panic!("Incorrect token should not be able to be encoded")
        }
    }

    #[test]
    async fn decode_from_request_with_correct_token() {
        let encoded_token: String = JwtToken::encode(32);
        let req = test::TestRequest::default()
            .append_header(("user-token", encoded_token))
            .to_http_request();
        let outcome = JwtToken::decode_from_request(req);
        match outcome {
            Ok(token) => assert_eq!(32, token.user_id),
            _ => panic!("Token is not returned when it should be")
        }
    }

    #[test]
    async fn decode_from_request_with_no_token() {
        let req = test::TestRequest::default()
            .append_header(("test", "test"))
            .to_http_request();
        let outcome = JwtToken::decode_from_request(req);
        match outcome {
            Err(message) => assert_eq!("No token received", message),
            _ => panic!("Token should not be returned when it is not present in the headers")
        }
    }

    #[test]
    async fn decode_from_request_with_incorrect_token() {
        let req = test::TestRequest::default()
            .append_header(("user-token", "test"))
            .to_http_request();
        let outcome = JwtToken::decode_from_request(req);
        match outcome {
            Err(message) => assert_eq!("Could not decode", message),
            _ => panic!("Should be an error in case incorrect token is received")
        }
    }
}