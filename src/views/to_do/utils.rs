use serde_json::{Map, Value};

use crate::{
    json_serialization::to_do_items::ToDoItems,
    state::read_file,
    to_do::{to_do_factory, ItemTypes},
};

use crate::diesel;
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::schema::to_do;

pub fn return_state(user_id: &i32) -> ToDoItems {
    let mut connection = establish_connection();

    let items = to_do::table
        .order(to_do::columns::id.asc())
        .filter(to_do::columns::user_id.eq(&user_id))
        .load::<Item>(&mut connection)
        .unwrap();

    let mut array_buffer = Vec::new();

    for item in items {
        let item: ItemTypes = to_do_factory(&item.status, &item.title).unwrap();
        array_buffer.push(item);
    }
    ToDoItems::new(array_buffer)
}
