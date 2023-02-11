use actix_web::{web, HttpResponse};
use serde_json::{Map, Value};

use crate::{
    json_serialization::to_do_item::ToDoItem, processes::process_input, state::read_file,
    to_do::to_do_factory,
};

use super::utils::return_state;

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");

    let title: String = to_do_item.title.clone();
    let status: String = to_do_item.status.clone();

    match to_do_factory(&status, &title) {
        Err(_) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
        Ok(item) => process_input(item, String::from("delete"), &state),
    }

    HttpResponse::Ok().json(return_state())
}
