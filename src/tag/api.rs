use poem_openapi::OpenApi;
use poem_openapi::param::{Path};
use poem_openapi::payload::Json;
use crate::tag::data::{GetTagResponse, GetTagsResponse, PatchTagRequest};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/v1/tags", method = "get")]
    async fn getAll(&self) -> Json<GetTagsResponse> {
        let response = GetTagsResponse {
            tags: Vec::new(),
        };

        return Json(response);
    }

    #[oai(path = "/v1/tags/:id", method = "get")]
    async fn get(&self) -> Json<GetTagResponse> {
        let response = GetTagResponse {
            statistics: vec![],
            status: 0,
            name: "".to_string(),
            text: "".to_string(),
        };

        return Json(response);
    }

    #[oai(path = "/v1/tags/:id", method = "patch")]
    async fn patch(&self, id: Path<String>, request: Json<PatchTagRequest>) {

    }
}