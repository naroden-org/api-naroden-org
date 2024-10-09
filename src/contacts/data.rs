use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};
use crate::error::data::ErrorResponse;


#[derive(ApiResponse)]
pub(crate) enum GetContactsResponse {
    #[oai(status = 200)]
    Ok(Json<GetContacts>),

    #[oai(status = 500)]
    GeneralError(Json<ErrorResponse>),
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetContacts {
    pub contacts: Vec<Contact>,
}

#[derive(Object, Deserialize, Serialize)]
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

#[derive(Object, Serialize, Deserialize)]
pub struct DbContactPhone {
    pub user_id: String,
    pub phone: String,
    pub nickname: String,
}