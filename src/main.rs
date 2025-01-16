mod web;
mod data;
mod error;
mod context;
mod service;
mod contract;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting api.naroden.org v0.1.4");
    println!("service calls: http://localhost:3001");
    println!("documentation: http://localhost:3001/docs");

    data::database::signin().await;
    data::database::execute_initial_script().await;
    web::server::start().await;

    Ok(())
}


