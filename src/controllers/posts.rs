use services::posts;
use db::models::{PostShort, Post, NewPost};
use rocket_contrib::{JSON};
use std::vec::Vec;
use rocket::response::status;

#[get("/posts")]
fn get_posts() -> JSON<Vec<PostShort>> {
    JSON(posts::get_posts())
}

#[get("/post/<post_id>")]
fn get_post(post_id: i64) -> Option<JSON<Post>> {
    let post = posts::get_post(post_id);
    match post {
        Ok(post_result) => Some(JSON(post_result)),
        Err(_) => None
    }
}

#[post("/post", data = "<json_post>")]
fn create_post(json_post: JSON<NewPost>) -> status::Created<Option<String>> {
    let post: NewPost = json_post.into_inner();
    posts::create_post(&post);
    status::Created("/post".to_string(), None)
}
