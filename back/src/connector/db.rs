use crate::adapters::{self, sqlx::Connector};
use futures::Future;

pub async fn init(database_url: &str) -> impl Future<Output = Connector> + '_ {
    adapters::sqlx::connect(database_url)
}
