mod web;
mod data;
mod error;
mod context;
mod service;
mod contract;

use tracing::info;
use tracing_appender::rolling;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting api.naroden.org v0.1.8");
    println!("service calls: http://localhost:3001");

    let info_file = rolling::daily("./log", "api-naroden-org");

    tracing_subscriber::fmt()
        .json()
        .with_writer(info_file)
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .init();

    info!("Starting server...");

    data::database::signin().await;
    data::database::execute_initial_script().await;
    web::server::start().await;

    Ok(())
}


