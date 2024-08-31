use poem_openapi::Object;
use serde::Serialize;

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetSurveysResponse {
    pub surveys: Vec<String>,
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetSurveyResponse {
    pub questions: Vec<Question>,
}

#[derive(Object, Serialize)]
pub struct Question {
    pub id: String,
    pub question: String,
    #[oai(rename = "type")]
    pub question_type: String,
    pub answers: Vec<String>,
}