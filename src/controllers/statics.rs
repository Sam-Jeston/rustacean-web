use rocket_contrib::{JSON, Value};
use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("app/index.html")
}

#[get("/node_modules/<file..>")]
fn module_files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("app/node_modules/").join(file)).ok()
}

#[get("/dist/<file..>")]
fn custom_files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("app/dist/").join(file)).ok()
}

#[error(404)]
fn not_found() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}
