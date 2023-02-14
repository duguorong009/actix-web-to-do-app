extern crate bcrypt;

use bcrypt::{hash, DEFAULT_COST};
use diesel::Insertable;
use uuid::Uuid;

use crate::schema::users;

#[derive(Insertable, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl NewUser {
    pub fn new(username: String, email: String, password: String) -> NewUser {
        let password: String = hash(password.as_str(), DEFAULT_COST).unwrap();
        let uuid = Uuid::new_v4().to_string();
        NewUser {
            username,
            email,
            password,
            unique_id: uuid,
        }
    }
}
