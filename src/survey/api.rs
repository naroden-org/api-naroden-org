use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::survey::data::{GetSurveyResponse, GetAllSurveysResponse, PostSurveyAnswerRequest, Survey, AllSurveys};

pub struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("USER")]
    #[oai(path = "/v1/surveys", method = "get")]
    async fn get_all(&self, db: Data<&Surreal<Client>>, raw_request: &Request) -> Result<GetAllSurveysResponse> {
        let response = AllSurveys {
            surveys: Vec::new(),
        };

        Ok(GetAllSurveysResponse::Ok(Json(response)))
    }

    #[protect("USER")]
    #[oai(path = "/v1/surveys/:id", method = "get")]
    async fn get(&self, db: Data<&Surreal<Client>>, raw_request: &Request, id: Path<String>) -> Result<GetSurveyResponse> {
        let response = Survey {
            questions: Vec::new(),
        };

        Ok(GetSurveyResponse::Ok(Json(response)))
    }

    #[protect("USER")]
    #[oai(path = "/v1/surveys/:id/questions/:question_id", method = "post")]
    async fn post_survey_answer(&self, db: Data<&Surreal<Client>>, raw_request: &Request, id: Path<String>, question_id: Path<String>, request: Json<PostSurveyAnswerRequest>) {

    }

}