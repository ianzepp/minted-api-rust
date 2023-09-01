// src/main.rs
#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate rocket;

use rocket::response::status;
mod helper;
mod record;
mod routers_data; // Import the new file

// Routes

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