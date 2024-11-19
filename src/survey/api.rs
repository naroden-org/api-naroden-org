use std::str::FromStr;
use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::survey::data::{GetSurveyResponse, GetAllSurveysResponse, PostSurveyAnswerRequest, Survey, AllSurveys, DbSurveyInfo, SurveyInfo, DbQuestion, Question};

pub struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("USER")]
    #[oai(path = "/private/v1/surveys", method = "get")]
    async fn get_all(&self, db: Data<&Surreal<Client>>) -> Result<GetAllSurveysResponse> {
        let surveys: Option<Vec<DbSurveyInfo>> = db.select("survey").await.ok().take();

        let mut data: Vec<SurveyInfo> = vec![];
        for db_survey in &surveys.unwrap() {
            data.push(SurveyInfo {
                id: db_survey.id.id.to_string(),
                name: db_survey.name.clone(),
                section: "Анкета".to_string(),
            });
        }

        let response = AllSurveys {
            surveys: data,
        };

        Ok(GetAllSurveysResponse::Ok(Json(response)))
    }

    #[protect("USER")]
    #[oai(path = "/private/v1/surveys/:id", method = "get")]
    async fn get(&self, db: Data<&Surreal<Client>>, id: Path<String>) -> Result<GetSurveyResponse> {

        let survey_id: Thing = Thing::from_str(format!("survey:{}", id.0).as_str()).unwrap();
        let questions: Vec<DbQuestion> = db.query(GET_QUESTIONS_FOR_SURVEY)
            .bind(("survey_id", survey_id))
            .await.expect("error").take(0).expect("error");

        let mut data: Vec<Question> = vec![];
        for db_question in &questions {
            data.push(Question {
                id: db_question.id.id.to_string(),
                text: db_question.text.clone(),
                r#type: db_question.r#type.clone(),
                options: if db_question.options.is_some() { db_question.options.clone()} else { None },
                answers: None, // TODO: once answers are implemented
                editable: db_question.editable,
            });
        }

        let response = Survey {
            questions: data,
        };

        Ok(GetSurveyResponse::Ok(Json(response)))
    }

    #[protect("USER")]
    #[oai(path = "/private/v1/surveys/:id/questions/:question_id", method = "post")]
    async fn post_survey_answer(&self, _db: Data<&Surreal<Client>>, _raw_request: &Request, _id: Path<String>, _question_id: Path<String>, _request: Json<PostSurveyAnswerRequest>) {}

}

const GET_QUESTIONS_FOR_SURVEY: &str = "select * from question where survey_id = $survey_id;";
