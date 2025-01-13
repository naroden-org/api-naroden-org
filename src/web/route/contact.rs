use std::str::FromStr;
use axum::Json;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::context::NarodenContext;
use crate::data::contact::DbContactPhone;
use crate::data::database::NARODEN_DB;
use crate::data::user::DbUser;
use crate::web::route::user::normalize_phone_number;
use crate::web::server::NarodenResult;

pub async fn retrieve_contacts(context: NarodenContext) -> NarodenResult<Json<GetContacts>> {
    let contacts: Vec<Contact> = NARODEN_DB.query(MATCH_CONTACTS)
        .bind(("$user_id_value", context.user_id().to_string()))
        .await.expect("error").take(0).expect("error");

    let response = GetContacts {
        contacts: contacts,
    };

    Ok(Json(response))
}

pub async fn create_contact(context: NarodenContext, request: Json<PostContactsRequest>) {
    let user_id: Thing = Thing::from_str(format!("user:{}", context.user_id()).as_str()).unwrap();
    let user: Option<DbUser> = NARODEN_DB.query(GET_USER_INFO)
        .bind(("user", user_id))
        .await.expect("error").take(0).expect("error");
    let phone_code: String = match user.unwrap().phone_code {
        None => {
            "".to_string()
        }
        Some(p) => {
            p.to_string()
        }
    };


    let mut data: Vec<DbContactPhone> = vec![];
    for contact in &request.phones {
        data.push(DbContactPhone {
            user_id: context.user_id().to_owned(),
            normalized_phone: normalize_phone_number(&contact.phone, &phone_code),
            phone: contact.phone.clone(),
            nickname: contact.nickname.clone(),
        });
    }

    // TODO: make it transactional
    NARODEN_DB.query("delete from contact_phone where user_id=$user_id")
        .bind(("user_id", context.user_id().to_string()))
        .await.ok();
    NARODEN_DB.insert::<Vec<DbContactPhone>>("contact_phone")
        .content(data)
        .await.ok();
}



#[derive(Debug, Serialize, Deserialize)]
pub struct GetContacts {
    pub contacts: Vec<Contact>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub phone: String,
    pub section: String,
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostContactsRequest {
    pub phones: Vec<ContactRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactRequest {
    pub phone: String,
    pub nickname: String,
}


pub const MATCH_CONTACTS: &str = "
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

const GET_USER_INFO: &str = "
    SELECT
        ->owns_contact[WHERE is_phone]->contact.value[0][0] as phone,
        ->owns_contact[WHERE is_email]->contact.value[0][0] as email,
        first_name,
        last_name,
        phone_code
    FROM ONLY $user;
";