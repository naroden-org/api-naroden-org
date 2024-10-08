use std::str::FromStr;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use super::data::{DbUser, GetUserResponse, PostUserRequest, User, UserResponse};
use poem_openapi::OpenApi;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::error::data::{create_error, Error};
use crate::jwt::data::{Jwt, JwtClaims, PostJwtResponse};
use crate::jwt::data::PostJwtResponse::NotFound;
use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::payload::Json;
use surrealdb::sql::Thing;
use crate::jwt::service::issue_jwt;
use crate::user::query::CREATE_USER_QUERY;

pub struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("NONE")]
    #[oai(path = "/v1/users", method = "post")]
    async fn create_user(&self, db: Data<&Surreal<Client>>, request: Json<PostUserRequest>) -> Result<PostJwtResponse> {
        let user: User = self.create_new_user(&db, &request).await;
        let jwt: Option<Jwt> = issue_jwt(user.id.unwrap().id.to_string());

        match jwt {
            Some(jwt) => Ok(PostJwtResponse::Ok(Json(Jwt::from(jwt)))),
            None => Ok(NotFound(Json(create_error(Error::InvalidCredentials)))),
        }
    }

    async fn create_new_user(&self, db: &Data<&Surreal<Client>>, user: &Json<PostUserRequest>) -> User {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(user.password.as_ref(), &salt)
            .ok()
            .expect("error")
            .to_string();

        let email: String = match &user.email {
            None => { "".to_string() }
            Some(email) => { email.to_string() }
        };

        let phone: String = match &user.phone {
            None => { "".to_string() }
            Some(phone) => { phone.to_string() }
        };

        let phone_code: i32 = match &user.phone_code {
            None => { 0 }
            Some(phone_code) => { phone_code.to_owned() }
        };

        let query = db
            .query(CREATE_USER_QUERY)
            .bind(("first_name", user.first_name.to_owned()))
            .bind(("last_name", user.last_name.to_owned()))
            .bind(("password", password_hash))
            .bind(("password_salt", salt.to_string()))
            .bind(("email", email))
            .bind(("phone", phone))
            .bind(("phone_code", phone_code));

        let created: Option<User> = query.await.expect("error").take(0).expect("no users returned from db");

        created.expect("failed to create a user")
    }


    #[protect("USER")]
    #[oai(path = "/v1/users", method = "get")]
    async fn get(&self, db: Data<&Surreal<Client>>, raw_request: &Request) -> Result<GetUserResponse> {
        let claims = raw_request.extensions().get::<JwtClaims>().unwrap();
        let user_id: Thing = Thing::from_str(format!("user:{}", claims.sub.to_owned()).as_str()).unwrap();

        let user: Option<DbUser> = db.query(GET_USER_INFO)
            .bind(("user", user_id))
            .await.expect("error").take(0).expect("error");


        match user {
            None => Ok(GetUserResponse::NotFound(Json(create_error(Error::GeneralError)))),
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
        phone_cone
    FROM ONLY $user;
";
