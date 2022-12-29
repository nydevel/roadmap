use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod adapters;
mod connector;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // TODO: remove this block to separate adapter
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:changeme@localhost/test")
        .await
        .unwrap();

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("some value: {:?}", row);

    let addr = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|e| panic!("Failed to get env with name 'SERVER_ADDRESS': {:?}", e));
    let routes_list = vec![routes::root(), routes::node()];

    connector::server::run(addr, routes_list).await;
}
