use db::models::*;
use diesel::prelude::*;
use diesel::insert;
use db;
use std::vec::Vec;

pub fn get_posts() -> Vec<PostShort> {
    use db::schema::posts::dsl::*;

    let connection = db::connection::establish_connection();
    let results = posts.select((id, title, caption, created_at, updated_at))
        .order(id.desc())
        .load::<PostShort>(&connection)
        .expect("Error loading posts");

    results
}

pub fn get_post(post_id: i64) -> Result<Post, String>  {
    use db::schema::posts::dsl::*;

    let connection = db::connection::establish_connection();
    let results = posts.filter(id.eq(post_id))
        .load::<Post>(&connection)
        .expect("Error loading post");

    if results.len() == 0 {
        Err("Post does not exist".to_string())
    } else {
        Ok(results[0].clone())
    }
}

pub fn create_post<'a>(new_post: &'a NewPost) -> () {
    use db::schema::posts;
    let connection = db::connection::establish_connection();
    let _ = insert(new_post).into(posts::table).execute(&connection);
    // Again this is a mysql downfall, lines should work...
    // .get_result(&connection)
    // .expect("Error saving new post")
}
