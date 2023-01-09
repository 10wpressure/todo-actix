extern crate bcrypt;

use uuid::Uuid;
use diesel::Insertable;
use bcrypt::{DEFAULT_COST, hash};
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
    pub fn new(username: String, email: String, password: String) -> Self {
        let hashed_password: String = hash(password.as_str(), DEFAULT_COST).expect("Could not hash the password");
        let uuid = Uuid::new_v4().to_string();
        Self {
            username,
            email,
            password: hashed_password,
            unique_id: uuid,
        }
    }
}