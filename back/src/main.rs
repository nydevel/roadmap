use dotenv::dotenv;
use std::env;

mod adapters;
mod connector;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let addr = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|e| panic!("Failed to get env with name 'SERVER_ADDRESS': {:?}", e));
    let routes_list = vec![routes::root(), routes::node()];

    connector::server::run(addr, routes_list).await;
}
