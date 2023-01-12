use super::super::user::generic_user::User;
use crate::schema::to_do;
use diesel::{Associations, Identifiable, Queryable};

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = to_do)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub user_id: i32,
}
