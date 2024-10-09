use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::error::data::ErrorResponse;

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

#[derive(Object, Serialize, Deserialize, Clone)]
#[oai(rename_all = "camelCase")]
pub struct PostUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub phone_code: Option<i32>,
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

#[derive(ApiResponse)]
pub(crate) enum GetUserResponse {
    #[oai(status = 200)]
    Ok(Json<UserResponse>),

    #[oai(status = 400)]
    NotFound(Json<ErrorResponse>),
}

#[derive(Object)]
#[oai(rename_all = "camelCase", skip_serializing_if_is_none = true)]
pub struct UserResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub phone_code: Option<i32>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DbUser {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    // TODO: move phone code to contact.code
    pub phone_code: Option<i32>,
}