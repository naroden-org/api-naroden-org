use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
pub struct DbAnswer {
    pub id: Thing,
    pub answers: Vec<String>,
    pub user_id: Thing,
    pub question_id: Thing,
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
pub struct DbSurveyInfo {
    pub id: Thing,
    pub name: String,
    pub questions: Vec<Thing>,
}