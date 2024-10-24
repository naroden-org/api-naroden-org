use super::data::{Jwt, PostJwtRequest, PostJwtResponse};
use super::service::issue;
use crate::error::data::{create_error, ApiError};
use poem::web::Data;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tracing::info;

pub struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("NONE")]
    #[oai(path = "/public/v1/jwt", method = "post")]
    async fn create(&self, db: Data<&Surreal<Client>>, request: Json<PostJwtRequest>) -> PostJwtResponse {
        let jwt: Option<Jwt> = issue(db, request.clone()).await;

        info!("{}", serde_json::to_string(&request.0).unwrap());

        match jwt {
            Some(jwt) => PostJwtResponse::Ok(Json(jwt)),
            None => PostJwtResponse::BadRequest(Json(create_error(ApiError::InvalidCredentials))),
        }
    }
}
