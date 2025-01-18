use serde::{Deserialize, Serialize};
use crate::data::database::NARODEN_DB;

pub async fn get_user_contacts(user_id: &str) -> Vec<DbContact> {
    NARODEN_DB.query(MATCH_CONTACTS)
        .bind(("$user_id_value", user_id.to_string()))
        .await.expect("error").take(0).expect("error")
}

const MATCH_CONTACTS: &str = "
{
   let $user_id = $user_id_value;
   let $registered_phones = array::intersect(
        (select normalized_phone from contact_phone where user_id=$user_id).normalized_phone,
        (select (value) as value from contact).value
   );
   return select
        nickname,
        phone,
        (IF normalized_phone in  $registered_phones THEN 'Регистрирани' ELSE 'Не регистрирани' END) as section
    from contact_phone
    where user_id = $user_id;
}
    ";

pub async fn store_user_contacts(user_id: &str, data: Vec<DbContactPhone>) {
    // TODO: make it transactional
    NARODEN_DB.query("delete from contact_phone where user_id=$user_id")
        .bind(("user_id", user_id.to_string()))
        .await.ok();
    NARODEN_DB.insert::<Vec<DbContactPhone>>("contact_phone")
        .content(data)
        .await.ok();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbContact {
    pub phone: String,
    pub section: String,
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbContactPhone {
    pub user_id: String,
    pub phone: String,
    pub normalized_phone: String,
    pub nickname: String,
}