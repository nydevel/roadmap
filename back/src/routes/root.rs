use serde_json::json;

use crate::connector::routes::{RequestType, Route};

pub fn root() -> Route {
    Route {
        path: String::from("/"),
        action: RequestType::GET,
        exec,
    }
}

fn exec() -> String {
    println!("root exec");
    json!({"root": "root"}).to_string()
}
