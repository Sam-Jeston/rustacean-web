use services::posts;
use db::models::Post;
use rocket_contrib::{JSON};
use std::vec::Vec;

#[get("/posts")]
fn get_posts() -> JSON<Vec<Post>> {
    JSON(posts::get_posts())
}
