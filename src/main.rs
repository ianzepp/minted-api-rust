// src/main.rs
#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate rocket;

use rocket::response::status;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod helper;
mod routers_data;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/<path..>", rank = 2)]
fn not_found(path: std::path::PathBuf) -> status::NotFound<String> {
    status::NotFound(format!("Not found: {}", path.to_str().unwrap_or("Invalid path")))
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![routers_data::data_select_all]) // Use the function from the new file
        .mount("/api", routes![routers_data::data_select_one]) // Use the function from the new file
        .mount("/", routes![not_found])
}