pub type Routes = Vec<Route>;

pub struct Route {
    pub path: String,
    pub action: RequestType,
    pub content: fn() -> String,
}

pub enum RequestType {
    POST,
    GET,
    DELETE,
    PUSH,
}
