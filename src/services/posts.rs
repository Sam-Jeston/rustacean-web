use db::models::*;
use diesel::prelude::*;
use db;
use std::vec::Vec;

pub fn get_posts() -> Vec<PostShort> {
    use db::schema::posts::dsl::*;

    let connection = db::connection::establish_connection();
    let results = posts.select((id, title, caption, created_at, updated_at))
        .load::<PostShort>(&connection)
        .expect("Error loading Users");

    results
}
