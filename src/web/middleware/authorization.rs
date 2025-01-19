use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts};
use axum::http::{HeaderMap, HeaderValue, Request};
use axum::http::request::Parts;
use axum::response::Response;
use axum::middleware::Next;
use axum::RequestPartsExt;
use envconfig::Envconfig;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use crate::context::NarodenContext;
use crate::error::NarodenError;
use crate::web::route::jwt::JwtClaims;
use crate::web::server::NarodenResult;

#[derive(Envconfig)]
struct SecretConfig {
    #[envconfig(from = "JWT_HS256_KEY")] // twice
    pub jwt_hs256_key: String,
}

pub async fn authorize(context: NarodenResult<NarodenContext>, req: Request<Body>, next: Next) -> NarodenResult<Response> {
    context?;
    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for NarodenContext {
    type Rejection = NarodenError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> NarodenResult<Self> {
        let headers = parts.extract::<HeaderMap>().await.unwrap();

        // authorization
        let authorization_header = headers.get("authorization");
        authorization_header.ok_or(NarodenError::MissingAuthorizationHeader)?;
        let jwt_claims : JwtClaims = get_claims(authorization_header.unwrap());

        // device information
        let device_type = headers.get("x-device-type").unwrap().to_str().unwrap().to_string();
        let device_id = headers.get("x-device-id").unwrap().to_str().unwrap().to_string();

        Ok(NarodenContext::new(jwt_claims.sub, jwt_claims.role, device_id, device_type))
    }
}

fn get_claims(token: &HeaderValue) -> JwtClaims {
    let jwt: &str = &token.to_str().unwrap()[7..];

    let jwt_hs256_key = DecodingKey::from_secret(
        SecretConfig::init_from_env()
            .unwrap()
            .jwt_hs256_key
            .as_ref());

    decode::<JwtClaims>(&jwt, &jwt_hs256_key, &Validation::new(Algorithm::HS256))
        .unwrap()
        .claims
}