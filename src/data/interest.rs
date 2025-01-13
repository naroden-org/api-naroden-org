use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
pub struct DbInterest {
    pub id: Thing,
    pub name: String,
    pub description: String,
    pub default_status: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DbHasInterest {
    // pub r#in: Thing,
    pub out: Thing,
    pub status: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DbInterestStatistics {
    pub count: i32,
    pub status: i32,
}