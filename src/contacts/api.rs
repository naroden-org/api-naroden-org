use poem_openapi::OpenApi;
use poem_openapi::payload::Json;
use crate::contacts::data::{GetContactsRegisteredRequest, GetContactsRegisteredResponse};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/v1/contacts/registered", method = "get")]
    async fn get(&self, request: Json<GetContactsRegisteredRequest>) -> Json<GetContactsRegisteredResponse> {
        let response = GetContactsRegisteredResponse {
            registered: Vec::new()
        };

        return Json(response);
    }
}