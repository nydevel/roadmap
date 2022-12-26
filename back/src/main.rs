use dotenv::dotenv;
use std::env;

mod adapters;
mod routes;
mod connector;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let addr = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|e| panic!("Failed to get env with name 'SERVER_ADDRESS': {:?}", e));

    connector::server::run(addr, routes::root()).await;
}
