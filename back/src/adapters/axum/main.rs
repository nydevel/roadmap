use axum::{routing::get, Router};
use std::net::SocketAddr;

use crate::connector::routes::{Route, Routes};

pub async fn run(addr: String, routes: Routes) {
    let mut app = Router::new();

    for route in routes {
        let Route { path, exec, .. } = route;

        app = app.route(&path, get(move || async move { (exec)() }));
    }

    let parsed_address: SocketAddr = addr
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse socket address: {:?}", e));

    axum::Server::bind(&parsed_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
