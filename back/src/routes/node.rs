use serde_json::json;

use crate::connector::routes::{RequestType, Route};

pub fn node() -> Route {
    Route {
        path: String::from("/node"),
        action: RequestType::GET,
        exec,
    }
}

fn exec() -> String {
    println!("node exec");
    json!({"node": "node"}).to_string()
}
