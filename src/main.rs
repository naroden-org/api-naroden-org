use envconfig::Envconfig;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Result, Route, Server};
use poem_openapi::OpenApiService;
use surrealdb::engine::remote::ws::{Client, Wss};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

mod error;
mod form;
mod jwt;
mod user;

#[derive(Envconfig)]
struct SecretConfig {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "DB_USERNAME")]
    pub db_username: String,

    #[envconfig(from = "DB_PASSWORD")]
    pub db_password: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SecretConfig::init_from_env().unwrap();

    let db: Surreal<Client> = Surreal::new::<Wss>(config.db_host).await?;
    db.signin(Root {
        username: config.db_username.as_str(),
        password: config.db_password.as_str(),
    })
    .await?;

    db.use_ns("api").use_db("prod").await?;

    let sql_definitions = "
        DEFINE FIELD in ON TABLE owns TYPE record<user>;
        DEFINE FIELD out ON TABLE owns TYPE record<contact>;
        DEFINE INDEX unique_owns_out ON TABLE owns COLUMNS out UNIQUE;
        DEFINE INDEX unique_contact_value ON TABLE contact COLUMNS value UNIQUE;
    ";
    db.query(sql_definitions).await?;

    let apis = (user::api::Api, jwt::api::Api);
    let api_service = OpenApiService::new(apis, "api.naroden.org", "0.0.1");

    let server = api_service.server("http://localhost:3001");
    let swagger_ui = server.swagger_ui();
    let route = Route::new()
        .nest("/", server)
        .nest("/docs", swagger_ui)
        .with(Cors::new())
        .data(db);

    println!("Starting api.naroden.org v0.0.1");
    println!("service calls: http://localhost:3001");
    println!("documentation: http://localhost:3001/docs");

    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(route)
        .await?;

    Ok(())
}
