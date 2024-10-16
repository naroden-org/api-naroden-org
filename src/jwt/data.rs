use crate::error::data::ErrorResponse;
use poem_openapi::payload::Json;
use poem_openapi::{ApiResponse, Object};
use serde::{Deserialize, Serialize};

#[derive(ApiResponse)]
pub(crate) enum PostJwtResponse {
    #[oai(status = 200)]
    Ok(Json<Jwt>),

    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
}

#[derive(Object, Serialize, Deserialize, Clone)]
#[oai(rename_all = "camelCase")]
pub struct Jwt {
    pub token: String,
}

#[derive(Object, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub sub: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Object, Serialize, Deserialize, Clone)]
#[oai(rename_all = "camelCase")]
pub struct PostJwtRequest {
    pub user_identifier: String,
    pub password: String,
}

#[derive(strum_macros::Display)]
pub enum UserRole {
    ADMIN,
    USER,
    NONE,
}