use crate::auth::jwt::JwtToken;

use super::utils::return_state;
use actix_web::{web, HttpRequest, Responder};

pub async fn get(req: HttpRequest) -> impl Responder {
    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();
    web::Json(return_state(&token.user_id))
}
