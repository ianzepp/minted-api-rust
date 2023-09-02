// src/main.rs
#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate rocket;

use rocket::response::status;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod models;
mod helper;
mod routers;

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

// #[derive(Debug)]
// #[derive(Deserialize, Serialize)]
// pub struct HttpDataRes {
//     pub status: i32,
//     pub length: i32,
//     pub schema: Option<String>,
//     pub record: Option<String>,
//     pub result: Option<Value>,
// }

// let res: HttpDataRes = HttpDataRes {
//     status: 200,
//     length: 10,
//     schema: None,
//     record: None,
//     result: None,
// };

// // Done
// jsonify(res)

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()

    // Data API routers 
    .mount("/api", routes![routers::data_select_any::run]) // Use the function from the new file
    .mount("/api", routes![routers::data_select_one::run]) // Use the function from the new file
    .mount("/api", routes![routers::data_create_all::run]) // Use the function from the new file
    .mount("/api", routes![routers::data_update_all::run]) // Use the function from the new file
    .mount("/api", routes![routers::data_update_one::run]) // Use the function from the new file
    .mount("/api", routes![routers::data_delete_all::run]) // Use the function from the new file
    .mount("/api", routes![routers::data_delete_one::run]) // Use the function from the new file

    // Test API routers
    .mount("/api", routes![routers::test_diesel::run]) // Use the function from the new file
    .mount("/api", routes![routers::test_ping::run]) // Use the function from the new file

    // 404 not found
    .mount("/", routes![not_found])
}