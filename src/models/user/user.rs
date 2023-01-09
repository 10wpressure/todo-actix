use diesel::{Queryable, Identifiable};
use bcrypt::verify;
use crate::schema::users;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl User {
    pub fn verify(self, password: String) -> bool {
        verify(password.as_str(), &self.password)
            .expect("Could not verify the password hash")
    }
}