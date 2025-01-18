use std::str::FromStr;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::data::database::NARODEN_DB;

pub async fn get_user_info(user_id: &str) -> Option<DbUser>  {
    let user_id: Thing = Thing::from_str(format!("user:{}", user_id).as_str()).unwrap();

    NARODEN_DB.query(GET_USER_INFO)
        .bind(("user", user_id))
        .await.expect("error").take(0).expect("error")
}

const GET_USER_INFO: &str = "
    SELECT
        ->owns_contact[WHERE is_phone]->contact.value[0][0] as phone,
        ->owns_contact[WHERE is_email]->contact.value[0][0] as email,
        first_name,
        last_name,
        phone_code
    FROM ONLY $user;
";


#[derive(Debug, Serialize, Deserialize)]
pub struct DbUser {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub phone_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Thing>,
    pub user_name: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub password: Option<String>,
    pub password_salt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: Option<Thing>,
    pub value: String,
    pub r#type: ContactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContactType {
    EMAIL,
    PHONE,
}