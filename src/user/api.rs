use super::data::{GetUserRequest, PostUserRequest, User, UserResponse};
use super::service::{create, find_all};
use poem::web::{Data, Query};
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/v1/users", method = "post")]
    async fn create(&self, db: Data<&Surreal<Client>>, request: Json<PostUserRequest>) -> Json<UserResponse> {
        let user: User = create(db.0, request.0.into()).await;

        Json(UserResponse::from(user))
    }

    #[oai(path = "/v1/users", method = "get")]
    async fn get(&self, request: Query<GetUserRequest>) -> Json<Vec<UserResponse>> {
        let users: Vec<User> = find_all(request.0);

        let response: Vec<UserResponse> = users
            .iter()
            .map(|c| UserResponse::from(c.clone()))
            .collect();

        Json(response)
    }
}
