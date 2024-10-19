use std::str::FromStr;
use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::{OpenApi};
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::contact::data::{Contact, DbContactPhone, GetContacts, GetContactsResponse, PostContactsRequest};
use crate::jwt::data::JwtClaims;

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

        let mut data: Vec<DbContactPhone> = vec![];
        for contact in &request.phones {
            // TODO: sanitize contacts phone number
            data.push(DbContactPhone {
                user_id: claims.sub.clone(),
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

pub const MATCH_CONTACTS: &str = "
{
   let $user_id = $user_id_value;
   let $registered_phones = array::intersect(
        (select phone from contact_phone where user_id=$user_id).phone,
        (select (value) as value from contact).value
   );
   return select
        nickname,
        phone,
        (IF phone in  $registered_phones THEN 'Регистрирани' ELSE 'Не регистрирани' END) as section
    from contact_phone
    where user_id = $user_id;
}
    ";