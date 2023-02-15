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
        let key: Hmac<Sha256> = Mac::new_from_slice(b"secret").unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id);
        let token_str: String = claims.sign_with_key(&key).unwrap();
        token_str
    }

    pub fn decode(encoded_token: String) -> Result<JwtToken, &'static str> {
        let key: Hmac<Sha256> = Mac::new_from_slice(b"secret").unwrap();
        let token_str: &str = encoded_token.as_str();

        let token: Result<Token<Header, BTreeMap<String, i32>, _>, _> =
            VerifyWithKey::verify_with_key(token_str, &key);
        match token {
            Ok(token) => {
                let _header = token.header();
                let claims = token.claims();
                Ok(JwtToken {
                    user_id: claims["user_id"],
                    body: encoded_token,
                })
            }
            Err(_) => Err("Could not decode"),
        }
    }

    pub fn decode_from_request(request: HttpRequest) -> Result<JwtToken, &'static str> {
        match request.headers().get("user-token") {
            Some(token) => JwtToken::decode(String::from(token.to_str().unwrap())),
            None => Err("there is no token"),
        }
    }
}

#[cfg(test)]
mod jwt_tests {
    use super::*;
    use actix_web::test;

    #[test]
    async fn encode_decode() {
        let encoded_token: String = JwtToken::encode(32);
        let decoded_token: JwtToken = JwtToken::decode(encoded_token).unwrap();
        assert_eq!(32, decoded_token.user_id);
    }

    #[test]
    async fn decode_incorrect_token() {
        let encoded_token: String = String::from("test");

        match JwtToken::decode(encoded_token) {
            Err(m) => assert_eq!("Could not decode", m),
            _ => panic!("Incorrect token should not be able to be encoded"),
        }
    }

    #[test]
    async fn decode_from_request_with_correct_token() {
        let encoded_token: String = JwtToken::encode(32);
        let request = test::TestRequest::default()
            .append_header((String::from("user-token"), encoded_token))
            .to_http_request();
        let res = JwtToken::decode_from_request(request);

        match res {
            Ok(token) => assert_eq!(32, token.user_id),
            _ => panic!("Token is not returned as it should be"),
        }
    }

    #[test]
    async fn decode_from_request_with_no_token() {
        let request = test::TestRequest::default()
            .append_header(("test", "test"))
            .to_http_request();
        let res = JwtToken::decode_from_request(request);

        match res {
            Err(m) => assert_eq!(m, "there is no token"),
            _ => panic!("Token should not be returned when it is not present in the headers"),
        }
    }

    #[test]
    async fn decode_from_request_with_false_token() {
        let request = test::TestRequest::default()
            .append_header(("user-token", "test"))
            .to_http_request();
        let out_come = JwtToken::decode_from_request(request);
        match out_come {
            Err(message) => assert_eq!("Could not decode", message),
            _ => panic!("should be an error with a fake token"),
        }
    }
}
