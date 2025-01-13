use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Clone)]
pub struct DbNews {
    pub id: Thing,
    pub title: String,
    pub text: String,
    pub image: String,
    pub buttons: Vec<DbNewsButton>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DbNewsButton {
    pub r#type: String,
    pub url: String,
}