use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::error::data::ErrorResponse;


#[derive(ApiResponse)]
pub(crate) enum GetAllFeedResponse {
    #[oai(status = 200)]
    Ok(Json<GetAllFeed>),

    #[oai(status = 500)]
    GeneralError(Json<ErrorResponse>),
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetAllFeed {
    pub feed: Vec<FeedItem>,
}

#[derive(Serialize, Deserialize)]
pub struct DbFeed {
    pub id: Thing,
    pub title: String,
    pub text: String,
    pub image: String,
    pub buttons: Vec<FeedButton>,
}

#[derive(Object, Deserialize, Serialize)]
pub struct FeedItem {
    pub id: String,
    pub title: String,
    pub text: String,
    pub image: String,
    pub buttons: Vec<FeedButton>,
}

#[derive(Object, Deserialize, Serialize, Clone)]
#[oai(rename_all = "camelCase")]
pub struct FeedButton {
    pub r#type: String,
    pub url: String,
}

#[derive(ApiResponse)]
pub(crate) enum GetFeedDetailsResponse {
    #[oai(status = 200)]
    Ok(Json<FeedDetails>),

    #[oai(status = 500)]
    GeneralError(Json<ErrorResponse>),
}

#[derive(Object, Deserialize, Serialize)]
pub struct FeedDetails {
    pub title: String,
    pub text: String,
    pub image: String,
    pub buttons: Vec<FeedButton>,
}
