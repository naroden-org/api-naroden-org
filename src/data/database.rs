use std::sync::LazyLock;
use envconfig::Envconfig;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub static NARODEN_DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[derive(Envconfig)]
struct DbSecretConfig {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "DB_USERNAME")]
    pub db_username: String,

    #[envconfig(from = "DB_PASSWORD")]
    pub db_password: String,
}

pub async fn signin() {
    let config = DbSecretConfig::init_from_env().unwrap();

    if config.db_host.contains("localhost") {
        NARODEN_DB.connect::<Ws>(config.db_host)
            .await.expect("Could not connect to db");
    } else {
        NARODEN_DB.connect::<Wss>(config.db_host)
            .await.expect("Could not connect to db");
    };

    NARODEN_DB.signin(Root {
        username: config.db_username.as_str(),
        password: config.db_password.as_str() })
        .await.expect("Could not sign in");

    NARODEN_DB
        .use_ns("api")
        .use_db("prod")
        .await.expect("Could not use namespace & database");
}

pub async fn execute_initial_script() {
    let sql_definitions = "
        DEFINE FIELD in ON TABLE owns_contact TYPE record<user>;
        DEFINE FIELD out ON TABLE owns_contact TYPE record<contact>;
        DEFINE INDEX unique_owns_contact_out ON TABLE owns_contact COLUMNS out UNIQUE;
        DEFINE INDEX unique_contact_value ON TABLE contact COLUMNS value UNIQUE;

        DEFINE FIELD in ON TABLE has_interest TYPE record<user>;
        DEFINE FIELD out ON TABLE has_interest TYPE record<interest>;
        DEFINE INDEX unique_user_and_interest ON TABLE has_interest COLUMNS in, out UNIQUE;

        DEFINE INDEX unique_contact_phone_per_user ON TABLE contact_phone COLUMNS phone, user_id UNIQUE;
        DEFINE INDEX index_contact_phone_user ON TABLE contact_phone COLUMNS user_id;
        DEFINE INDEX index_contact_phone_phone ON TABLE contact_phone COLUMNS phone;
    ";

    NARODEN_DB.query(sql_definitions)
        .await
        .expect("Failed to create indexes and constraints when booting the application");
}