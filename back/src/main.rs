use axum::{response::Html, routing::get, Router};
use dotenv::dotenv;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/", get(root));
    let addr = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|e| panic!("Failed to get env with name 'SERVER_ADDRESS': {:?}", e));

    let parsed_address: SocketAddr = addr
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse socket address: {:?}", e));

    axum::Server::bind(&parsed_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html("hello world")
}
