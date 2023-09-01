#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate rocket;

use rocket::response::status;
use rocket::serde::json::json;

// Routes
#[get("/api/data/<schema>/<record>")]
fn data_select_all(schema: &str, record: &str) -> String {
    println!("Schema: {}", schema);
    println!("Record: {}", record);
    serde_json::to_string(&json!({"status": 200})).unwrap()
}

#[get("/<path..>", rank = 2)]
fn not_found(path: std::path::PathBuf) -> status::NotFound<String> {
    status::NotFound(format!("Not found: {}", path.to_str().unwrap_or("Invalid path")))
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![data_select_all])
        .mount("/", routes![not_found])
}