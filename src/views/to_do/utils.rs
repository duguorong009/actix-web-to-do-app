use serde_json::{Map, Value};

use crate::{
    json_serialization::to_do_items::ToDoItems,
    state::read_file,
    to_do::{to_do_factory, ItemTypes},
};

pub fn return_state() -> ToDoItems {
    let state: Map<String, Value> = read_file("./state.json");

    let mut array_buffer = Vec::new();

    for (key, value) in state {
        let item_type: String = String::from(value.as_str().unwrap());
        let item: ItemTypes = to_do_factory(&item_type, &key).unwrap();
        array_buffer.push(item);
    }
    let return_package: ToDoItems = ToDoItems::new(array_buffer);
    return_package
}
