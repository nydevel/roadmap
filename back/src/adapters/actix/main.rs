use std::sync::Arc;

use crate::connector::routes::Route;
use actix_web::{web, App, HttpServer};

pub async fn run(addr: String, route: Route) {
    let Route { path, content, .. } = route;

    HttpServer::new(|| App::new().service(web::resource("/").to(|| async { Arc::clone(content) })))
        .bind(addr)
        .unwrap()
        .run()
        .await;
}
