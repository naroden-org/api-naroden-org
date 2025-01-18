use std::str::FromStr;
use axum::extract::Path;
use axum::Json;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::context::NarodenContext;
use crate::data::database::NARODEN_DB;
use crate::data::model::survey::{DbAnswer, DbQuestion, DbSurveyInfo};
use crate::web::server::NarodenResult;

pub async fn retrieve_all_surveys() -> NarodenResult<Json<AllSurveys>> {
    let surveys: Option<Vec<DbSurveyInfo>> = NARODEN_DB.select("survey").await.ok().take();

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

    Ok(Json(response))
}

pub async fn retrieve_survey(context: NarodenContext, Path(id): Path<String>) -> NarodenResult<Json<Survey>> {
    let user_id: Thing = Thing::from_str(format!("user:{}", context.user_id()).as_str()).unwrap();
    let survey_id: Thing = Thing::from_str(format!("survey:{}", id).as_str()).unwrap();

    let mut query = NARODEN_DB.query(GET_QUESTIONS_FOR_SURVEY)
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

    Ok(Json(response))
}

pub async fn create_survey_answer(context: NarodenContext, Path(id): Path<String>, request: Json<PostSurveyAnswerRequest>) {
    let user_id: Thing = Thing::from_str(format!("user:{}", context.user_id()).as_str()).unwrap();
    let question_id: Thing = Thing::from_str(format!("question:{}", id).as_str()).unwrap();

    NARODEN_DB.query(UPSERT_ANSWER)
        .bind(("user_id", user_id))
        .bind(("question_id", question_id))
        .bind(("answers", request.answers.clone()))
        .await.ok();
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllSurveys {
    pub surveys: Vec<SurveyInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurveyInfo {
    pub id: String,
    pub name: String,
    pub section: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Survey {
    pub questions: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub id: String,
    pub text: String,
    pub r#type: String,
    pub options: Option<Vec<String>>,
    pub answers: Option<Vec<String>>,
    pub editable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostSurveyAnswerRequest {
    pub answers: Vec<String>,
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