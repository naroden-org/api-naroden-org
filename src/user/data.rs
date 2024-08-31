use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: Option<Thing>,
    pub user_name: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub password: Option<String>,
    pub password_salt: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Contact {
    pub id: Option<Thing>,
    pub value: String,
    pub r#type: ContactType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ContactType {
    EMAIL,
    PHONE,
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct PostUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: String,
    pub referral: Option<String>,
}

#[derive(Object, Deserialize)]
#[oai(rename_all = "camelCase")]
pub struct GetUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub referral: Option<String>,
}

#[derive(Object)]
#[oai(rename_all = "camelCase")]
pub struct UserResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub phone_code: Option<i32>
}
