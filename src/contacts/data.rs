use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetContactsResponse {
    pub contacts: Vec<Contact>,
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct Contact {
    pub phone: String,
    pub section: String,
    pub nickname: String,
}

#[derive(Object, Deserialize)]
#[oai(rename_all = "camelCase")]
pub struct PostContactsRequest {
    pub phones: Vec<ContactRequest>,
}

#[derive(Object, Deserialize)]
#[oai(rename_all = "camelCase")]
pub struct ContactRequest {
    pub phone: String,
    pub nickname: String,
}