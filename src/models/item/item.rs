use super::super::user::user::User;
use crate::schema::to_do;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = to_do)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub user_id: i32,
}
