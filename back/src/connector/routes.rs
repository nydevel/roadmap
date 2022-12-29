pub type Routes = Vec<Route>;

pub struct Route {
    pub path: String,
    pub action: RequestType,
    pub exec: fn() -> String,
}

pub enum RequestType {
    POST,
    GET,
    DELETE,
    PUSH,
}
