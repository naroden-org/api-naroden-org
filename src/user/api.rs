use std::collections::HashMap;
use std::str::FromStr;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use super::data::{DbUser, GetUserResponse, PostUserRequest, User, UserResponse};
use poem_openapi::OpenApi;
use surrealdb::engine::remote::ws::Client;
use surrealdb::{Error, Surreal};
use crate::error::data::{create_error, ApiError};
use crate::jwt::data::{Jwt, JwtClaims, PostJwtResponse};
use crate::jwt::data::PostJwtResponse::BadRequest;
use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::payload::Json;
use surrealdb::sql::Thing;
use crate::contact::api::normalize_phone_number;
use crate::jwt::service::issue_jwt;
use crate::user::query::CREATE_USER_QUERY;

pub struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("NONE")]
    #[oai(path = "/public/v1/users", method = "post")]
    async fn create_user(&self, db: Data<&Surreal<Client>>, request: Json<PostUserRequest>) -> Result<PostJwtResponse> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(request.password.as_ref(), &salt)
            .ok()
            .expect("error")
            .to_string();

        let email: String = match &request.email {
            None => { "".to_string() }
            Some(email) => { email.to_string() }
        };

        let phone_code: String = match &request.phone_code {
            None => { "".to_string() }
            Some(phone_code) => { phone_code.to_string() }
        };

        let phone: String = match &request.phone {
            None => { "".to_string() }
            Some(phone) => {
                normalize_phone_number(phone, &phone_code)
            }
        };

        let mut create_user_response = db
            .query(CREATE_USER_QUERY)
            .bind(("first_name", request.first_name.to_owned()))
            .bind(("last_name", request.last_name.to_owned()))
            .bind(("password", password_hash))
            .bind(("password_salt", salt.to_string()))
            .bind(("email", email.to_string()))
            .bind(("phone", phone.to_string()))
            .bind(("phone_code", phone_code))
            .await;

        let mut create_user_response = match create_user_response {
            Ok(response) => {
                response
            }
            Err(error) => {
                dbg!(error);
                return Ok(BadRequest(Json(create_error(ApiError::GeneralError))))
            }
        };

        let data:HashMap<usize, Error> = create_user_response.take_errors();
        if !data.is_empty() {
            match serde_json::to_string(&data) {
                Ok(error) => {
                    if error.contains("Database index `unique_contact_value` already contains")
                    {
                        if email.len() > 0  && error.contains(&email) {
                            return Ok(BadRequest(Json(create_error(ApiError::AlreadyUsedEmail))))
                        }
                        if phone.len() > 0 && error.contains(&phone) {
                            return Ok(BadRequest(Json(create_error(ApiError::AlreadyUsedPhone))))
                        }
                    }
                }
                Err(e) => {
                    // event!(Level::ERROR, e);
                    dbg!(e);
                    return Ok(BadRequest(Json(create_error(ApiError::GeneralError))))
                }
            }
        }

        let user: Option<User> = create_user_response.take(0).expect("reason");

        let jwt: Option<Jwt> = issue_jwt(user.unwrap().id.unwrap().id.to_string());

        // TODO: send email on registration
        // TODO: accept terms and conditions
        // TODO: validate phone code

        match jwt {
            Some(jwt) => Ok(PostJwtResponse::Ok(Json(Jwt::from(jwt)))),
            None => Ok(BadRequest(Json(create_error(ApiError::InvalidCredentials)))),
        }
    }

    #[protect("USER")]
    #[oai(path = "/private/v1/profile", method = "get")]
    async fn get(&self, db: Data<&Surreal<Client>>, raw_request: &Request) -> Result<GetUserResponse> {
        let claims: &JwtClaims = raw_request.extensions().get::<JwtClaims>().unwrap();
        let user_id: Thing = Thing::from_str(format!("user:{}", claims.sub.to_owned()).as_str()).unwrap();

        let user: Option<DbUser> = db.query(GET_USER_INFO)
            .bind(("user", user_id))
            .await.expect("error").take(0).expect("error");



        match user {
            None => Ok(GetUserResponse::NotFound(Json(create_error(ApiError::GeneralError)))),
            Some(user) => {
                let response = UserResponse {
                    first_name: user.first_name,
                    last_name: user.last_name,
                    email: user.email,
                    phone: user.phone,
                    phone_code: user.phone_code,
                };

                Ok(GetUserResponse::Ok(Json(response)))
            }
        }
    }
}

const GET_USER_INFO: &str = "
    SELECT
        ->owns_contact[WHERE is_phone]->contact.value[0][0] as phone,
        ->owns_contact[WHERE is_email]->contact.value[0][0] as email,
        first_name,
        last_name,
        phone_code
    FROM ONLY $user;
";
