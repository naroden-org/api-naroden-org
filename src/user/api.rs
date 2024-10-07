use super::data::{PostUserRequest, User, UserResponse};
use super::service::{create};
use poem::web::{Data};
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::jwt::data::{Jwt, PostJwtResponse};

pub struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("NONE")]
    #[oai(path = "/v1/users", method = "post")]
    async fn create(&self, db: Data<&Surreal<Client>>, request: Json<PostUserRequest>) -> PostJwtResponse {
        let user: User = create(db.0, request.0.into()).await;

        let jwt = Jwt {
            token: "example".to_string(),
        };

        PostJwtResponse::Ok(Json(Jwt::from(jwt)))
    }

    #[oai(path = "/v1/users", method = "get")]
    async fn get(&self) -> Json<UserResponse> {
        let response = UserResponse {
            first_name: "example".to_string(),
            last_name: "example".to_string(),
            email: None,
            phone: None,
            phone_code: None,
        };

        Json(response)
    }
}
