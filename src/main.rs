#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate dotenv;
extern crate rocket;

mod db;
mod controllers;
mod services;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            controllers::statics::index,
            controllers::users::get_user
        ])
        .catch(errors![controllers::statics::not_found])
        .launch();
}
