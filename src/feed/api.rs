use poem_openapi::OpenApi;
use poem_openapi::param::{Query};
use poem_openapi::payload::Json;
use crate::feed::data::{GetFeedResponse};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/v1/feed", method = "get")]
    async fn get(&self, id: Query<Option<String>>, count: Query<Option<i32>>) -> Json<GetFeedResponse> {
        let response = GetFeedResponse {
            feed: Vec::new(),
        };

        return Json(response);
    }
}