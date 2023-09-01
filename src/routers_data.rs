// src/routers-data.rs
use rocket::get;
use rocket::serde::json::json;
use serde_json::to_string;

#[get("/data/<schema>")]
pub fn data_select_all(schema: &str) -> String {
    println!("Schema: {}", schema);
    to_string(&json!({"status": 200, "schema": schema, "result": []})).unwrap()
}

#[get("/data/<schema>/<record>")]
pub fn data_select_one(schema: &str, record: &str) -> String {
    println!("Schema: {}", schema);
    println!("Record: {}", record);
    to_string(&json!({"status": 200, "schema": schema, "result": record})).unwrap()
}
