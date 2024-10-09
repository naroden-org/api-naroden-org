use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use crate::jwt::data::{Jwt, JwtClaims, PostJwtRequest, UserRole};
use crate::user::data::User;
use chrono::{offset, Days};
use envconfig::Envconfig;
use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use poem::web::Data;
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[derive(Envconfig)]
struct SecretConfig {
    #[envconfig(from = "JWT_HS256_KEY")] // twice
    pub jwt_hs256_key: String,
}

pub async fn issue(db: Data<&Surreal<Client>>, credentials: Json<PostJwtRequest>) -> Option<Jwt> {
    let get_user_password_info = "
        SELECT *
        FROM ONLY (SELECT <-owns_contact<-user as id
                   FROM contact
                   WHERE value = $contact)[0].id[0];
    ";

    let user: Option<User> = db
        .query(get_user_password_info)
        .bind(("contact", credentials.user_identifier.clone()))
        .await.expect("error").take(0).expect("error");

    if user.is_none() {
        return None;
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
        panic!("wrong password!")
    }

    issue_jwt(user.id.unwrap().id.to_string())
}

pub fn issue_jwt(user_id: String) -> Option<Jwt> {
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

    Option::from(Jwt { token: token.unwrap() })
}
