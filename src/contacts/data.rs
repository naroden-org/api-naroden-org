use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetContactsRegisteredResponse {
    pub registered: Vec<String>,
}

#[derive(Object, Deserialize)]
#[oai(rename_all = "camelCase")]
pub struct GetContactsRegisteredRequest {
    pub phones: Vec<String>,
}