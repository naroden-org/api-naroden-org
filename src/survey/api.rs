use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use crate::survey::data::{GetSurveyResponse, GetSurveysResponse};

pub struct Api;

#[OpenApi]
impl Api {

    #[oai(path = "/v1/surveys", method = "get")]
    async fn get_all(&self) -> Json<GetSurveysResponse> {
        let response = GetSurveysResponse {
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

}