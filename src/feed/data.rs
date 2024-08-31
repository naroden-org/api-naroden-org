use poem_openapi::Object;
use serde::{Serialize};

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetFeedResponse {
    pub feed: Vec<Feed>,
}

#[derive(Object, Serialize)]
pub struct Feed {
    pub id: String,
    pub title: String,
    pub text: String,
    pub image: String,
    pub buttons: Vec<FeedButton>,
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct FeedButton {
    #[oai(rename = "type")]
    pub button_type: String,
    pub url: String,
}