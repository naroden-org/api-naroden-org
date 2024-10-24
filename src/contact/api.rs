use std::str::FromStr;
use std::string::String;
use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::{OpenApi};
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::contact::data::{Contact, DbContactPhone, GetContacts, GetContactsResponse, PostContactsRequest};
use crate::jwt::data::JwtClaims;
use crate::user::data::DbUser;

pub struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {

    #[protect("USER")]
    #[oai(path = "/private/v1/contacts", method = "get")]
    async fn get(&self, db: Data<&Surreal<Client>>, raw_request: &Request) -> Result<GetContactsResponse> {
        let claims = raw_request.extensions().get::<JwtClaims>().unwrap();

        let contacts: Vec<Contact> = db.query(MATCH_CONTACTS)
            .bind(("$user_id_value", claims.sub.to_owned()))
            .await.expect("error").take(0).expect("error");

        let response = GetContacts {
            contacts: contacts,
        };

        Ok(GetContactsResponse::Ok(Json(response)))
    }

    #[protect("USER")]
    #[oai(path = "/private/v1/contacts", method = "post")]
    async fn post(&self, db: Data<&Surreal<Client>>, raw_request: &Request, request: Json<PostContactsRequest>) {
        let claims = raw_request.extensions().get::<JwtClaims>().unwrap();
        let user_id: Thing = Thing::from_str(format!("user:{}", claims.sub.to_owned()).as_str()).unwrap();
        let user: Option<DbUser> = db.query(GET_USER_INFO)
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
            // TODO: sanitize contacts phone number

            data.push(DbContactPhone {
                user_id: claims.sub.clone(),
                normalized_phone: normalize_phone_number(&contact.phone, &phone_code),
                phone: contact.phone.clone(),
                nickname: contact.nickname.clone(),
            });
        }

        // TODO: make it transactional
       db.query("delete from contact_phone where user_id=$user_id")
            .bind(("user_id", claims.sub.to_owned()))
            .await.ok();
        db.insert::<Vec<DbContactPhone>>("contact_phone")
            .content(data)
            .await.ok();
    }
}

   pub fn normalize_phone_number(phone: &String, phone_code: &String) -> String {
       let mut normalized_phone = phone.replace("+", "")
           .replace(" ", "")
           .replace("-", "")
           .replace("(", "")
           .replace(")", "");

       if normalized_phone.starts_with("00") {
           normalized_phone = normalized_phone.strip_prefix("00").unwrap().to_string()
       } else if normalized_phone.starts_with("0") {
           normalized_phone = normalized_phone.replacen("0", phone_code, 1);
       }

       normalized_phone
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