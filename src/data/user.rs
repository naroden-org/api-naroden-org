use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct DbUser {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub phone_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Thing>,
    pub user_name: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub password: Option<String>,
    pub password_salt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: Option<Thing>,
    pub value: String,
    pub r#type: ContactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContactType {
    EMAIL,
    PHONE,
}