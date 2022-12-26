use axum::{routing::get, Router};
use std::net::SocketAddr;

use crate::connector::routes::Route;

pub async fn run(addr: String, route: Route) {
    let Route { path, content, .. } = route;

    let app = Router::new().route(&path, get(|| async { content }));

    let parsed_address: SocketAddr = addr
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse socket address: {:?}", e));

    axum::Server::bind(&parsed_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
