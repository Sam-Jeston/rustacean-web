use services::users;
use db::models::User;
use rocket_contrib::{JSON};

#[get("/users/<username>")]
fn get_user(username: &str) -> Option<JSON<User>> {
    let user: Result<User, &str> = users::get_user(username.to_string());

    match user {
        Ok(user_res) => Some(JSON(user_res)),
        Err(_) => None
    }
}
