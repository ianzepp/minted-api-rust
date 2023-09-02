// src/routers-data.rs
use rocket::get;
use rocket::serde::json::Value;
use serde::{Deserialize, Serialize};

// Local imports
use crate::helper::jsonify;

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct HttpDataRes {
    pub status: i32,
    pub length: i32,
    pub schema: Option<String>,
    pub record: Option<String>,
    pub result: Option<Value>,
}

#[get("/data/<_schema_name>")]
pub fn data_select_all(_schema_name: String) -> String {
    let res: HttpDataRes = HttpDataRes {
        status: 200,
        length: 10,
        schema: None,
        record: None,
        result: None,
    };

    // Done
    jsonify(res)
}

#[get("/data/<_schema_name>/<_record_id>")]
pub fn data_select_one(_schema_name: String, _record_id: String) -> String {
    let res: HttpDataRes = HttpDataRes {
        status: 200,
        length: 10,
        schema: None,
        record: None,
        result: None,
    };

    // Done
    jsonify(res)
}
