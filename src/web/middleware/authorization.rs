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
use tracing::info;
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
        let device_provider = get_header_value("x-device-provider", &headers);
        let device_id = get_header_value("x-device-id", &headers);

        let device_model = get_header_value("x-device-model", &headers);
        let device_os = get_header_value("x-device-os", &headers);
        let naroden_version = get_header_value("x-naroden-version", &headers);
        info!("x-device-model [{}] x-device-os [{}] x-naroden-version [{}]", device_model, device_os, naroden_version);

        Ok(NarodenContext::new(jwt_claims.sub, jwt_claims.role, device_id, device_provider))
    }
}

fn get_header_value(header: &str, headers: &HeaderMap) -> String {
    let result = headers.get(header);

    if result.is_none() {
        String::from("")
    } else {
        result.unwrap().to_str().unwrap().to_string()
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