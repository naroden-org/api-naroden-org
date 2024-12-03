use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
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
    pub id: String,
    pub name: String,
    pub section: String,
}

#[derive(Serialize, Deserialize)]
pub struct DbSurveyInfo {
    pub id: Thing,
    pub name: String,
    pub questions: Vec<Thing>,
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
    pub r#type: String,
    pub options: Option<Vec<String>>,
    pub answers: Option<Vec<String>>,
    pub editable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DbQuestion {
    pub id: Thing,
    pub text: String,
    pub r#type: String,
    pub options: Option<Vec<String>>,
    pub editable: bool,
    pub survey_id: Thing,
}

#[derive(Serialize, Deserialize)]
pub struct DbAnswer {
    pub id: Thing,
    pub answers: Vec<String>,
    pub user_id: Thing,
    pub question_id: Thing,
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct PostSurveyAnswerRequest {
    pub answers: Vec<String>,
}