use crate::error::data::ErrorResponse;
use poem_openapi::payload::Json;
use poem_openapi::{ApiResponse, Object};
use serde::{Deserialize, Serialize};

#[derive(ApiResponse)]
pub(crate) enum PostJwtResponse {
    #[oai(status = 200)]
    Ok(Json<Jwt>),

    #[oai(status = 400)]
    NotFound(Json<ErrorResponse>),
}

#[derive(Object)]
#[oai(rename_all = "camelCase")]
pub struct Jwt {
    pub token: String,
}

#[derive(Object, Serialize, Deserialize)]
pub struct JwtPayload {
    pub sub: String,
    pub exp: i64,
}

#[derive(Object)]
#[oai(rename_all = "camelCase")]
pub struct PostJwtRequest {
    pub user_identifier: String,
    pub password: String,
}
