use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};
use crate::error::data::ErrorResponse;


#[derive(ApiResponse)]
pub(crate) enum GetStatisticsResponse {
    #[oai(status = 200)]
    Ok(Json<GetStatistics>),

    #[oai(status = 500)]
    GeneralError(Json<ErrorResponse>),
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetStatistics {
    pub stats: Vec<Statistic>,
    pub new_users: Vec<Statistic>,
}

#[derive(Object, Deserialize, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct Statistic {
    pub description: String,
    pub value: String,
}