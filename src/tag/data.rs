use poem_openapi::{Object};
use serde::{Deserialize, Serialize};

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetTagsResponse {
    pub tags: Vec<Tag>,
}

#[derive(Object, Serialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub section: String,
}

#[derive(Object, Deserialize)]
#[oai(rename_all = "camelCase")]
pub struct PatchTagRequest {
    pub status: i32,
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetTagResponse {
    pub statistics: Vec<Tag>,
    pub status: i32,
    pub name: String,
    pub text: String,
}

#[derive(Object, Serialize)]
pub struct Statistics {
    pub section: String,
    pub enabled: String,
    pub disabled: String,
    pub neutral: String,
}