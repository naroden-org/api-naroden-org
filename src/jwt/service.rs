use crate::jwt::data::{Jwt, JwtClaims, PostJwtRequest, UserRole};
use crate::user::data::User;
use chrono::{offset, Days};
use envconfig::Envconfig;
use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[derive(Envconfig)]
struct SecretConfig {
    #[envconfig(from = "JWT_HS256_KEY")] // twice
    pub jwt_hs256_key: String,
}

pub async fn issue(db: &Surreal<Client>, credentials: PostJwtRequest) -> Option<Jwt> {
    let get_user_password_info = "
        SELECT *
        FROM ONLY (SELECT <-owns_contact<-user as id
                   FROM contact
                   WHERE value = $contact)[0].id[0];
    ";

    let mut result = db
        .query(get_user_password_info)
        .bind(("contact", credentials.user_identifier))
        .await
        .ok()?;

    let user: Option<User> = result.take(0).ok()?;

    if user.is_none() {
        return None;
    }

    let claims = JwtClaims {
        sub: user.unwrap().id.unwrap().id.to_string(),
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
