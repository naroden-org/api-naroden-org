use super::data::{Jwt, PostJwtRequest, PostJwtResponse};
use super::service::issue;
use crate::error::data::{create_error, ApiError};
use crate::jwt::data::PostJwtResponse::BadRequest;
use poem::web::Data;
use poem::Result;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("NONE")]
    #[oai(path = "/v1/jwt", method = "post")]
    async fn create(&self, db: Data<&Surreal<Client>>, request: Json<PostJwtRequest>) -> Result<PostJwtResponse> {
        let jwt: Option<Jwt> = issue(db, request.clone()).await;

        match jwt {
            Some(jwt) => Ok(PostJwtResponse::Ok(Json(Jwt::from(jwt)))),
            None => Ok(BadRequest(Json(create_error(ApiError::InvalidCredentials)))),
        }
    }
}
