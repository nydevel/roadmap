use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type Connector = Pool<Postgres>;

pub async fn connect(database_url: &str) -> Connector {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .unwrap()
}
