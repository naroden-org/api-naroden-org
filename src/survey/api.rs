use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use crate::survey::data::{GetSurveyResponse, GetAllSurveysResponse, PostSurveyAnswerRequest};
use crate::user::data::PostUserRequest;

pub struct Api;

#[OpenApi]
impl Api {

    #[oai(path = "/v1/surveys", method = "get")]
    async fn get_all(&self) -> Json<GetAllSurveysResponse> {
        let response = GetAllSurveysResponse {
            surveys: Vec::new(),
        };

        return Json(response);
    }

    #[oai(path = "/v1/surveys/:id", method = "get")]
    async fn get(&self, id: Path<String>) -> Json<GetSurveyResponse> {
        let response = GetSurveyResponse {
            questions: Vec::new(),
        };

        return Json(response);
    }

    #[oai(path = "/v1/surveys/:id/questions/:question_id", method = "post")]
    async fn post_survey_answer(&self, id: Path<String>, question_id: Path<String>, request: Json<PostSurveyAnswerRequest>) {
        let response = GetSurveyResponse {
            questions: Vec::new(),
        };
    }

}