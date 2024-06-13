use super::data::{Jwt, PostJwtRequest, PostJwtResponse};
use super::service::issue;
use crate::error::data::{create_error, Error};
use crate::jwt::data::PostJwtResponse::NotFound;
use poem::web::Data;
use poem::Result;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/v1/jwt", method = "post")]
    async fn create(&self, db: Data<&Surreal<Client>>, request: Json<PostJwtRequest>) -> Result<PostJwtResponse> {
        let jwt: Option<Jwt> = issue(db.0, request.0).await;

        match jwt {
            Some(jwt) => Ok(PostJwtResponse::Ok(Json(Jwt::from(jwt)))),
            None => Ok(NotFound(Json(create_error(Error::InvalidCredentials)))),
        }
    }
}
