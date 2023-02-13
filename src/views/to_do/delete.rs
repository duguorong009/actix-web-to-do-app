use actix_web::{web, HttpResponse};

use crate::json_serialization::to_do_item::ToDoItem;

use crate::diesel;
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::schema::to_do;

use super::utils::return_state;

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let title: String = to_do_item.title.clone();

    let mut connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();

    let _ = diesel::delete(&items[0]).execute(&mut connection);

    HttpResponse::Ok().json(return_state())
}
