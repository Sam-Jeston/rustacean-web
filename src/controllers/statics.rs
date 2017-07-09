use rocket_contrib::{JSON, Value};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[error(404)]
fn not_found() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}
