use argon2::password_hash::SaltString;
use axum::Json;
use envconfig::Envconfig;
use serde::{Deserialize, Serialize};
use argon2::{Argon2, PasswordHasher};
use chrono::{offset, Days};
use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use crate::data::database::NARODEN_DB;
use crate::data::user::User;
use crate::error::NarodenError;
use crate::web::server::NarodenResult;

#[derive(Envconfig)]
struct SecretConfig {
    #[envconfig(from = "JWT_HS256_KEY")] // twice
    pub jwt_hs256_key: String,
}

pub async fn issue_jwt(credentials: Json<PostJwtRequest>) -> NarodenResult<Json<Jwt>> {
    let get_user_password_info = "
        SELECT *
        FROM ONLY (SELECT <-owns_contact<-user as id
                   FROM contact
                   WHERE value = $contact)[0].id[0];
    ";

    let user: Option<User> = NARODEN_DB
        .query(get_user_password_info)
        .bind(("contact", credentials.user_identifier.clone()))
        .await.expect("error").take(0).expect("error");

    if user.is_none() {
        return Err(NarodenError::InvalidCredentials);
    }

    let user = user.unwrap();

    let salt = SaltString::from_b64(&*user.password_salt.unwrap()).unwrap();
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(credentials.password.as_ref(), &salt)
        .ok()
        .expect("error")
        .to_string();

    if password_hash != user.password.unwrap() {
        return Err(NarodenError::InvalidCredentials);
    }

    create_jwt(user.id.unwrap().id.to_string())
}

pub fn create_jwt(user_id: String) -> NarodenResult<Json<Jwt>> {
    let claims = JwtClaims {
        sub: user_id,
        role: UserRole::USER.to_string(),
        exp: offset::Utc::now()
            .checked_add_days(Days::new(30))
            .unwrap()
            .timestamp(),
        iat: offset::Utc::now().timestamp(),
    };

    // TODO: extract and initialize once
    let key = EncodingKey::from_secret(
        SecretConfig::init_from_env()
            .unwrap()
            .jwt_hs256_key
            .as_ref());

    let token = encode(&Header::new(Algorithm::HS256), &claims, &key);

    Ok(Json(Jwt { token: token.unwrap() }))
}



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Jwt {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtClaims {
    pub sub: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostJwtRequest {
    pub user_identifier: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(strum_macros::Display)]
pub enum UserRole {
    // ADMIN,
    USER,
    NONE,
}