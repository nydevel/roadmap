use crate::connector::routes::Route;

pub fn root() -> Route {
    Route {
        path: String::from("/"),
        action: String::from("fef"),
        content: String::from("result1"),
    }
}
