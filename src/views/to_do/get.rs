use super::utils::return_state;
use actix_web::{web, Responder};

pub async fn get() -> impl Responder {
    web::Json(return_state())
}
