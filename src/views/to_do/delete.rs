use actix_web::{web, HttpRequest, HttpResponse};

use crate::auth::jwt::JwtToken;
use crate::json_serialization::to_do_item::ToDoItem;

use crate::diesel;
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::schema::to_do;

use super::utils::return_state;

pub async fn delete(to_do_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {
    let title: String = to_do_item.title.clone();

    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();

    let mut connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(title.as_str()))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();

    let _ = diesel::delete(&items[0]).execute(&mut connection);

    HttpResponse::Ok().json(return_state(&token.user_id))
}
