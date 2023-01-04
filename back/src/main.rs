use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod adapters;
mod connector;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|e| panic!("Failed to get env with name 'SERVER_ADDRESS': {:?}", e));
    let addr = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|e| panic!("Failed to get env with name 'SERVER_ADDRESS': {:?}", e));

    let routes_list = vec![routes::root(), routes::node()];

    let connection = connector::db::init(&database_url).await;

    connector::server::run(addr, routes_list).await;
}
