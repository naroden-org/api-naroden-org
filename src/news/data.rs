use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::error::data::ErrorResponse;


#[derive(ApiResponse)]
pub(crate) enum GetAllNewsResponse {
    #[oai(status = 200)]
    Ok(Json<GetAllNews>),

    #[oai(status = 500)]
    GeneralError(Json<ErrorResponse>),
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetAllNews {
    pub news: Vec<NewsItem>,
}

#[derive(Serialize, Deserialize)]
pub struct DbNews {
    pub id: Thing,
    pub title: String,
    pub text: String,
    pub image: String,
    pub buttons: Vec<NewsButton>,
}

#[derive(Object, Deserialize, Serialize)]
pub struct NewsItem {
    pub id: String,
    pub title: String,
    pub text: String,
    pub image: String,
    pub buttons: Vec<NewsButton>,
}

#[derive(Object, Deserialize, Serialize, Clone)]
#[oai(rename_all = "camelCase")]
pub struct NewsButton {
    pub r#type: String,
    pub url: String,
}

#[derive(ApiResponse)]
pub(crate) enum GetNewsDetailsResponse {
    #[oai(status = 200)]
    Ok(Json<NewsDetails>),

    #[oai(status = 500)]
    GeneralError(Json<ErrorResponse>),
}

#[derive(Object, Deserialize, Serialize)]
pub struct NewsDetails {
    pub title: String,
    pub text: String,
    pub image: String,
    pub buttons: Vec<NewsButton>,
}
