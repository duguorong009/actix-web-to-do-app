use actix_web::{web, HttpResponse};
use serde_json::{Map, Value};

use crate::{
    json_serialization::to_do_item::ToDoItem, processes::process_input, state::read_file,
    to_do::to_do_factory,
};

use super::utils::return_state;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");

    let title_ref: &str = &to_do_item.title.clone();
    let title: String = to_do_item.title.clone();

    let status: String;
    match &state.get(title_ref) {
        Some(result) => status = result.to_string().replace('\"', ""),
        None => return HttpResponse::NotFound().json(format!("{} not in state", title_ref)),
    }

    if &status == &to_do_item.status {
        return HttpResponse::Ok().json(return_state());
    }

    match to_do_factory(&status, &title) {
        Err(_item) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
        Ok(item) => process_input(item, String::from("edit"), &state),
    }

    HttpResponse::Ok().json(return_state())
}
