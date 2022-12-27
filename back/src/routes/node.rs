use serde_json::json;

use crate::connector::routes::{RequestType, Route};

pub fn node() -> Route {
    Route {
        path: String::from("/node"),
        action: RequestType::GET,
        content: json!({"test": "test"}),
    }
}
