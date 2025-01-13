use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DbStatistic {
    pub description: String,
    pub value: i32,
}