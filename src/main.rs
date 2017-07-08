#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate rocket;

use db::models::*;
use diesel::prelude::*;

mod db;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    use db::schema::users::dsl::*;

    let connection = db::connection::establish_connection();
    let results = users.limit(5)
        .load::<User>(&connection)
        .expect("Error loading Users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.username);
        println!("----------\n");
        println!("{}", user.password);
    }

    rocket::ignite().mount("/", routes![index]).launch();
}
