pub type Routes = Vec<Route>;

// TODO: move exec results to json/object
// TODO: make exec async?? - for DB work
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
