mod root;

pub use root::*;

pub struct Route {
    pub path: String,
    pub action: String,
    pub content: String,
}
