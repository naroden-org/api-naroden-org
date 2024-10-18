use std::str::FromStr;
use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::{OpenApi};
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::jwt::data::JwtClaims;
use crate::partner::data::{GetPartners, GetPartnersResponse};

pub struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("USER")]
    #[oai(path = "/v1/partners", method = "get")]
    async fn get(&self, db: Data<&Surreal<Client>>, raw_request: &Request) -> Result<GetPartnersResponse> {
        let claims = raw_request.extensions().get::<JwtClaims>().unwrap();

        //let contacts: Vec<Contact> = db.query(MATCH_CONTACTS)
        //    .bind(("user_id", claims.sub.to_owned()))
        //    .await.expect("error").take(0).expect("error");

        let response = GetPartners {
            partners: vec![],
        };

        Ok(GetPartnersResponse::Ok(Json(response)))
    }
}

pub const MATCH_CONTACTS: &str = "
    select
        nickname,
        phone,
        (IF phone in array::intersect((select phone from contact_phones).phone, (select (value) as value from contact).value) THEN 'Регистрирани' ELSE 'Не регистрирани' END) as section
    from contact_phones
    where user_id = $user_id;
    ";