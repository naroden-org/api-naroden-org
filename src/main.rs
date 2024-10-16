use std::collections::HashSet;
use envconfig::Envconfig;
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Result, Route, Server, Request, IntoResponse};
use poem::http::StatusCode;
use poem::middleware::{CatchPanic, PanicHandler, Tracing, RequestId};
use poem_grants::GrantsMiddleware;
use poem_openapi::{OpenApiService};
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use crate::error::data::{create_error, ApiError};
use crate::jwt::data::{JwtClaims, UserRole};
use poem::Endpoint;
use tracing_subscriber::filter::LevelFilter;

mod error;
mod jwt;
mod user;
mod tag;
mod survey;
mod contacts;
mod statistics;
mod partners;
mod news;

#[derive(Envconfig)]
struct SecretConfig {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "DB_USERNAME")]
    pub db_username: String,

    #[envconfig(from = "DB_PASSWORD")]
    pub db_password: String,

    #[envconfig(from = "JWT_HS256_KEY")] // twice
    pub jwt_hs256_key: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SecretConfig::init_from_env().unwrap();
    let db: Surreal<Client> = if config.db_host.contains("localhost") {
        Surreal::new::<Ws>(config.db_host).await?
    } else {
        Surreal::new::<Wss>(config.db_host).await?
    };

    db.signin(Root {
        username: config.db_username.as_str(),
        password: config.db_password.as_str(),
    })
    .await?;

    db.use_ns("api").use_db("prod").await?;

    let sql_definitions = "
        DEFINE FIELD in ON TABLE owns_contact TYPE record<user>;
        DEFINE FIELD out ON TABLE owns_contact TYPE record<contact>;
        DEFINE INDEX unique_owns_contact_out ON TABLE owns_contact COLUMNS out UNIQUE;
        DEFINE INDEX unique_contact_value ON TABLE contact COLUMNS value UNIQUE;

        DEFINE FIELD in ON TABLE owns_tag TYPE record<user>;
        DEFINE FIELD out ON TABLE owns_tag TYPE record<tag>;
        DEFINE INDEX unique_user_and_tag ON TABLE owns_tag COLUMNS in, out UNIQUE;
    ";
    db.query(sql_definitions).await?;

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();


    let apis = (user::api::Api, jwt::api::Api, news::api::Api, tag::api::Api, survey::api::Api, contacts::api::Api, partners::api::Api, statistics::api::Api);
    let api_service = OpenApiService::new(apis, "api.naroden.org", "0.0.11");


    let panic_handler = CatchPanic::new().with_handler(|_| {
        Json(create_error(ApiError::GeneralError))
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
    });

    let server = api_service.server("https://api.naroden.org");
    let swagger_ui = server.swagger_ui();

    let server = server
        .with(GrantsMiddleware::with_extractor(extract))
        .with(Cors::new())
        .with(Tracing)
        .with(panic_handler)
        .with(RequestId::new());


    let route = Route::new()
        .nest("/", server)
        .nest("/docs", swagger_ui)
        .data(db);

    println!("Starting api.naroden.org v0.0.11");
    println!("service calls: http://localhost:3001");
    println!("documentation: http://localhost:3001/docs");

    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(route)
        .await?;

    Ok(())
}

async fn extract(req: &mut Request) -> Result<HashSet<String>> {
    let authorization_header = req.headers().get("authorization").cloned();
    match authorization_header {
        None => {
            Ok(HashSet::from([UserRole::NONE.to_string()]))
        }
        Some(token) => {
            let jwt: &str = &token.to_str().unwrap()[7..];

            let jwt_hs256_key = DecodingKey::from_secret(
                SecretConfig::init_from_env()
                    .unwrap()
                    .jwt_hs256_key
                    .as_ref());

            let claims = decode::<JwtClaims>(&jwt, &jwt_hs256_key, &Validation::new(Algorithm::HS256))
                .unwrap()
                .claims;

            req.extensions_mut().insert::<JwtClaims>(claims.clone());

            Ok(HashSet::from([claims.role]))
        }
    }
}

