use serde_json::Value;

pub type Routes = Vec<Route>;

pub struct Route {
    pub path: String,
    pub action: RequestType,
    pub content: Value,
}

pub enum RequestType {
    POST,
    GET,
    DELETE,
    PUSH,
}
