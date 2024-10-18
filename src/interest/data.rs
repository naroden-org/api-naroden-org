use poem::error::ResponseError;
use poem::http::StatusCode;
use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::error::data::{ErrorResponse};

#[derive(ApiResponse)]
pub(crate) enum GetInterestsResponse {
    #[oai(status = 200)]
    Ok(Json<GetInterests>),

    #[oai(status = 500)]
    GeneralError(Json<ErrorResponse>),
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetInterests {
    pub interests: Vec<Interest>,
}

#[derive(Object, Serialize)]
pub struct Interest {
    pub id: String,
    pub name: String,
    pub section: String,
}

#[derive(Serialize, Deserialize)]
pub struct DbInterest {
    pub id: Thing,
    pub name: String,
    pub section: String,
    pub text: String,
    pub default_status: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DbOwnsInterest {
    pub r#in: Thing,
    pub out: Thing,
    pub status: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DbInterestStatistics {
    pub count: i32,
    pub status: i32,
}

#[derive(Object, Deserialize)]
#[oai(rename_all = "camelCase")]
pub struct PatchInterestRequest {
    pub status: i32,
}

#[derive(ApiResponse)]
pub(crate) enum GetInterestResponse {
    #[oai(status = 200)]
    Ok(Json<GetInterest>),

    #[oai(status = 500)]
    GeneralError(Json<ErrorResponse>),
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetInterest {
    pub stats: Vec<Statistics>,
    pub status: i32,
    pub name: String,
    pub description: String,
}

#[derive(Object, Serialize)]
pub struct Statistics {
    pub section: String,
    pub allowed: i32,
    pub forbidden: i32,
    pub neutral: i32,
}

struct DbError(surrealdb::Error);

impl ResponseError for DbError {
    fn status(&self) -> StatusCode {
        todo!()
    }
}


