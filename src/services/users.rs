use db::models::*;
use diesel::prelude::*;
use db;

pub fn get_user<'a>(target_username: String) -> Result<User, &'a str> {
    use db::schema::users::dsl::*;

    let connection = db::connection::establish_connection();
    let results = users.filter(username.eq(target_username))
        .limit(1)
        .load::<User>(&connection)
        .expect("Error loading Users");

    if results.len() == 0 {
        Err("This user does not exist")
    } else {
        Ok(results[0].clone())
    }
}
