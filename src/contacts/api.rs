use poem_openapi::OpenApi;
use poem_openapi::payload::Json;
use crate::contacts::data::{GetContactsResponse, PostContactsRequest};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/v1/contacts", method = "get")]
    async fn get(&self) -> Json<GetContactsResponse> {
        let response = GetContactsResponse {
            contacts: vec![],
        };

        return Json(response);
    }

    #[oai(path = "/v1/contacts", method = "post")]
    async fn post(&self, request: Json<PostContactsRequest>) {}
}