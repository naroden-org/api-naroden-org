use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::data::database::NARODEN_DB;

pub async fn get_notification_token(token: &str) -> Option<DbNotificationToken> {
    let result: Option<DbNotificationToken> = NARODEN_DB.query(GET_USER_NOTIFICATION_FOR_DEVICE)
        .bind(("token_value", token.to_string()))
        .await.expect("error").take(0).expect("error");

    result
}

pub async fn store_notification_token(token: DbNotificationToken) {
    let _: Vec<DbNotificationToken> = NARODEN_DB.insert::<Vec<DbNotificationToken>>("notification_token")
        .content(vec![token])
        .await.expect("error");

}

pub async fn delete_notification_token(token_id: &Thing) {
    let _: Option<DbNotificationToken> = NARODEN_DB.delete(("notification_token", token_id.id.to_raw().to_string()))
        .await.expect("error");
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DbNotificationToken {
    pub id: Option<Thing>,
    pub device_type: String,
    pub token: String,
    pub device_id: String,
    pub user_id: String,
}

pub enum DeviceType {
    GOOGLE,
    IOS,
    HUAWEY,
}

impl DeviceType {
    pub fn device_type_id(&self) -> String {
        match *self {
            DeviceType::GOOGLE => { "1".to_string() }
            DeviceType::IOS => { "2".to_string() }
            DeviceType::HUAWEY => { "3".to_string() }
        }
    }
}

const GET_USER_NOTIFICATION_FOR_DEVICE: &str = "select * from notification_token where token = $token_value;";


