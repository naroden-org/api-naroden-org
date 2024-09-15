use poem_openapi::Object;
use serde::Serialize;

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetAllSurveysResponse {
    pub surveys: Vec<Survey>,
}

#[derive(Object, Serialize)]
pub struct Survey {
    pub name: String,
    pub section: String,
}

#[derive(Object, Serialize)]
#[oai(rename_all = "camelCase")]
pub struct GetSurveyResponse {
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