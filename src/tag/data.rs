use poem::error::ResponseError;
use poem::http::StatusCode;
use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::error::data::{ErrorResponse};

#[derive(ApiResponse)]
pub(crate) enum GetTagsResponse {
    #[oai(status = 200)]
    Ok(Json<GetTags>),

    #[oai(status = 500)]
    GeneralError(Json<ErrorResponse>),
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetTags {
    pub tags: Vec<Tag>,
}

#[derive(Object, Serialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub section: String,
}

#[derive(Serialize, Deserialize)]
pub struct DbTag {
    pub id: Thing,
    pub name: String,
    pub section: String,
    pub text: String,
    pub default_status: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DbOwnsTag {
    pub r#in: Thing,
    pub out: Thing,
    pub status: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DbTagStatistics {
    pub count: i32,
    pub status: i32,
}

#[derive(Object, Deserialize)]
#[oai(rename_all = "camelCase")]
pub struct PatchTagRequest {
    pub status: i32,
}

#[derive(ApiResponse)]
pub(crate) enum GetTagResponse {
    #[oai(status = 200)]
    Ok(Json<GetTag>),

    #[oai(status = 500)]
    GeneralError(Json<ErrorResponse>),
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetTag {
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


