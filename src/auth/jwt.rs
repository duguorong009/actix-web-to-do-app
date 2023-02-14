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