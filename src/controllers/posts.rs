use services::posts;
use db::models::PostShort;
use rocket_contrib::{JSON};
use std::vec::Vec;

#[get("/posts")]
fn get_posts() -> JSON<Vec<PostShort>> {
    JSON(posts::get_posts())
}
