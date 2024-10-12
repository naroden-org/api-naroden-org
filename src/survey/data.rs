use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use serde::Serialize;
use crate::error::data::ErrorResponse;

#[derive(ApiResponse)]
pub(crate) enum GetAllSurveysResponse {
    #[oai(status = 200)]
    Ok(Json<AllSurveys>),

    #[oai(status = 500)]
    GeneralError(Json<ErrorResponse>),
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct AllSurveys {
    pub surveys: Vec<SurveyInfo>,
}

#[derive(Object, Serialize)]
pub struct SurveyInfo {
    pub name: String,
    pub section: String,
}

#[derive(ApiResponse)]
pub(crate) enum GetSurveyResponse {
    #[oai(status = 200)]
    Ok(Json<Survey>),

    #[oai(status = 500)]
    GeneralError(Json<ErrorResponse>),
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct Survey {
    pub questions: Vec<Question>,
}

#[derive(Object, Serialize)]
pub struct Question {
    pub id: String,
    pub text: String,
    #[oai(rename = "type")]
    pub question_type: String,
    pub options: Vec<String>,
    pub answers: Option<Vec<String>>,
    pub editable: bool,
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct PostSurveyAnswerRequest {
    pub answers: Vec<String>,
}