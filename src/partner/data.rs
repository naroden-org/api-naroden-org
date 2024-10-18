use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};
use crate::error::data::ErrorResponse;


#[derive(ApiResponse)]
pub(crate) enum GetPartnersResponse {
    #[oai(status = 200)]
    Ok(Json<GetPartners>),

    #[oai(status = 500)]
    GeneralError(Json<ErrorResponse>),
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetPartners {
    pub partners: Vec<Partner>,
}

#[derive(Object, Deserialize, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct Partner {
    pub image_url: String,
    pub buttons: Vec<PartnerButton>,
}

#[derive(Object, Deserialize, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct PartnerButton {
    pub r#type: String,
    pub url: String,
}