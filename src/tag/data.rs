use poem_openapi::{Object};
use serde::{Deserialize, Serialize};

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetTagsResponse {
    pub feed: Vec<Tag>,
}

#[derive(Object, Serialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub status: i32,
}

#[derive(Object, Deserialize)]
#[oai(rename_all = "camelCase")]
pub struct PatchTagRequest {
    pub status: i32,
}