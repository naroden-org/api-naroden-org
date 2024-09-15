use poem_openapi::OpenApi;
use poem_openapi::param::{Query};
use poem_openapi::payload::Json;
use crate::feed::data::{FeedResponse, GetAllFeedResponse};

pub struct Api;

pub fn get_default_feed_count() -> i32 { 20 }

#[OpenApi]
impl Api {

    #[oai(path = "/v1/feed", method = "get")]
    async fn getAll(&self,
                    id: Query<Option<String>>,
                    #[oai(default = "get_default_feed_count")]
                    count: Query<i32>) -> Json<GetAllFeedResponse> {
        let response = GetAllFeedResponse {
            feed: Vec::new(),
        };

        return Json(response);
    }

    #[oai(path = "/v1/feed/:id", method = "get")]
    async fn get(&self, id: Query<String>) -> Json<FeedResponse>
    {
        let response = FeedResponse {
            title: "".to_string(),
            text: "".to_string(),
            image: "".to_string(),
            buttons: vec![],
        };

        return Json(response);
    }
}