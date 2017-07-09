use db::models::*;
use diesel::prelude::*;
use db;
use std::vec::Vec;

pub fn get_posts() -> Vec<Post> {
    use db::schema::posts::dsl::*;

    let connection = db::connection::establish_connection();
    let results = posts.load::<Post>(&connection)
        .expect("Error loading Users");

    results
}
