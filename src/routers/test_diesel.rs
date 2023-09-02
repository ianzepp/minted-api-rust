use rocket::get;
use rocket::serde::json::Json;
use diesel::RunQueryDsl;

use crate::models::record::Record;
use crate::models::record::RecordHash;
use crate::models::schema::SchemaType;
use crate::database::establish_connection;
use crate::models::schema::schema::dsl::*;

#[get("/test/diesel", format = "application/json")]
pub fn run() -> Json<Vec<Record>> {
    let mut conn = establish_connection();
    let results = schema
        .load::<SchemaType>(&mut conn)
        .expect("Error loading schemas");
    
    Json(results)
}