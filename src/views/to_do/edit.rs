use actix_web::{web, HttpRequest, HttpResponse};

use crate::auth::jwt::JwtToken;
use crate::json_serialization::to_do_item::ToDoItem;

use super::utils::return_state;

use crate::database::establish_connection;
use crate::diesel;
use crate::schema::to_do;
use diesel::prelude::*;

pub async fn edit(to_do_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {
    let title_ref: String = to_do_item.title.clone();

    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();

    let mut connection = establish_connection();
    let results = to_do::table
        .filter(to_do::columns::title.eq(title_ref))
        .filter(to_do::columns::user_id.eq(&token.user_id));

    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("done"))
        .execute(&mut connection);

    HttpResponse::Ok().json(return_state(&token.user_id))
}
