use std::str::FromStr;
use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::jwt::data::JwtClaims;
use crate::survey::data::{GetSurveyResponse, GetAllSurveysResponse, PostSurveyAnswerRequest, Survey, AllSurveys, DbSurveyInfo, SurveyInfo, DbQuestion, Question, DbAnswer};

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
    async fn get(&self, db: Data<&Surreal<Client>>, raw_request: &Request, id: Path<String>) -> Result<GetSurveyResponse> {

        let claims = raw_request.extensions().get::<JwtClaims>().unwrap();
        let user_id: Thing = Thing::from_str(format!("user:{}", claims.sub.to_owned()).as_str()).unwrap();
        let survey_id: Thing = Thing::from_str(format!("survey:{}", id.0).as_str()).unwrap();

        let mut query = db.query(GET_QUESTIONS_FOR_SURVEY)
            .bind(("survey_id", survey_id))
            .bind(("user_id", user_id))
            .await.expect("error");
        let questions: Vec<DbQuestion> = query.take(1).expect("error");
        let answers: Vec<DbAnswer> = query.take(2).expect("error");

        let mut data: Vec<Question> = vec![];
        for db_question in &questions {
            let answer_index: Option<usize> = answers.iter().position(|a:&DbAnswer| a.question_id == db_question.id);
            data.push(Question {
                id: db_question.id.id.to_string(),
                text: db_question.text.clone(),
                r#type: db_question.r#type.clone(),
                options: if db_question.options.is_some() { db_question.options.clone()} else { None },
                answers: if answer_index.is_some() { Some(answers[answer_index.unwrap()].answers.clone())} else {None },
                editable: db_question.editable,
            });
        }

        let response = Survey {
            questions: data,
        };

        Ok(GetSurveyResponse::Ok(Json(response)))
    }

    #[protect("USER")]
    #[oai(path = "/private/v1/questions/:id/answers", method = "post")]
    async fn post_survey_answer(&self, db: Data<&Surreal<Client>>, raw_request: &Request, id: Path<String>, request: Json<PostSurveyAnswerRequest>) {
        let claims = raw_request.extensions().get::<JwtClaims>().unwrap();
        let user_id: Thing = Thing::from_str(format!("user:{}", claims.sub.to_owned()).as_str()).unwrap();
        let question_id: Thing = Thing::from_str(format!("question:{}", id.0).as_str()).unwrap();

        db.query(UPSERT_ANSWER)
            .bind(("user_id", user_id))
            .bind(("question_id", question_id))
            .bind(("answers", request.answers.clone()))
            .await.ok();
    }

}

const GET_QUESTIONS_FOR_SURVEY: &str = "
    LET $questions = (select * from question where survey_id = $survey_id);
    $questions;
    select * from answer where question_id in $questions.id && user_id = $user_id;
";

const UPSERT_ANSWER: &str = "
    UPSERT answer SET
        user_id = $user_id,
        question_id = $question_id,
        answers = $answers
    WHERE question_id = $question_id && user_id = $user_id;
";