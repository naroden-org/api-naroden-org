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
    #[oai(path = "/private/v1/partners", method = "get")]
    async fn get(&self, _db: Data<&Surreal<Client>>, raw_request: &Request) -> Result<GetPartnersResponse> {
        let _claims = raw_request.extensions().get::<JwtClaims>().unwrap();

        let response = GetPartners {
            partners: vec![],
        };

        Ok(GetPartnersResponse::Ok(Json(response)))
    }
}