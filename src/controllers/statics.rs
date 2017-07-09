use rocket_contrib::{JSON, Value};
use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("app/index.html")
}

#[get("/about")]
fn about() -> io::Result<NamedFile> {
    NamedFile::open("app/index.html")
}

#[get("/post/<post_id>")]
fn post(post_id: i64) -> io::Result<NamedFile> {
    NamedFile::open("app/index.html")
}

#[get("/404")]
fn four_oh_four() -> io::Result<NamedFile> {
    NamedFile::open("app/index.html")
}

#[get("/dist/<file..>")]
fn dist_files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("app/dist/").join(file)).ok()
}

#[error(404)]
fn not_found() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}
