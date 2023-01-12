use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct NewUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}
