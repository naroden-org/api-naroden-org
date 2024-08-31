use poem_openapi::OpenApi;
use poem_openapi::param::{Path};
use poem_openapi::payload::Json;
use crate::tag::data::{GetTagsResponse, PatchTagRequest};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/v1/tags", method = "get")]
    async fn get(&self) -> Json<GetTagsResponse> {
        let response = GetTagsResponse {
            feed: Vec::new(),
        };

        return Json(response);
    }

    #[oai(path = "/v1/tags/:id", method = "patch")]
    async fn patch(&self, id: Path<String>, request: Json<PatchTagRequest>) {

    }
}