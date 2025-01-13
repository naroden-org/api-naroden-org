use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DbContactPhone {
    pub user_id: String,
    pub phone: String,
    pub normalized_phone: String,
    pub nickname: String,
}