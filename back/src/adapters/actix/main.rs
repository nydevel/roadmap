use crate::connector::routes::Route;
use actix_web::{web, App, HttpServer};

struct AppState {
    content: String,
}

async fn render(data: web::Data<AppState>) -> String {
    data.content.clone()
}

pub async fn run(addr: String, route: Route) {
    let Route { content, path, .. } = route;
    let st = web::Data::new(AppState { content });

    HttpServer::new(move || {
        App::new()
            .app_data(st.clone())
            .service(web::resource(&path).to(render))
    })
    .bind(addr)
    .unwrap()
    .run()
    .await
    .unwrap();
}
