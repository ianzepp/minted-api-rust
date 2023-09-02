use diesel::deserialize::FromSqlRow;
// src/routers-data.rs
use rocket::get;
use rocket::serde::json::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Local imports
use crate::helper::{jsonify, valuefy};
use crate::record::*;
use crate::establish_connection;


#[derive(Debug, Deserialize, Serialize)]
pub struct HttpDataRes {
    pub status: i32,
    pub length: i32,
    pub schema: Option<String>,
    pub record: Option<String>,
    pub result: Option<Value>,
}

#[get("/data/<fixme>")]
pub fn data_select_all(fixme: &str) -> String {
    // Placeholder array of RecordJson instances
    let mut record_json_array: Vec<RecordJson> = Vec::new();

    // Populate the array with placeholder RecordJson instances
    for _ in 0..10 {
        let record_json = RecordJson {
            meta: RecordMeta {
                id: Some(String::from("placeholder_id")),
                ns: Some(String::from("placeholder_ns")),
                sc: Some(String::from("placeholder_sc")),
            },
            data: HashMap::new(),
            info: RecordInfo {
                created_at: Some(String::from("placeholder_created_at")),
                created_by: Some(String::from("placeholder_created_by")),
                updated_at: Some(String::from("placeholder_updated_at")),
                updated_by: Some(String::from("placeholder_updated_by")),
                expired_at: Some(String::from("placeholder_expired_at")),
                expired_by: Some(String::from("placeholder_expired_by")),
                deleted_at: Some(String::from("placeholder_expired_by")),
                deleted_by: Some(String::from("placeholder_expired_by")),
            },
            acls: RecordAcls {
                acls_full: Vec::new(),
                acls_edit: Vec::new(),
                acls_read: Vec::new(),
                acls_deny: Vec::new(),
            },
        };

        record_json_array.push(record_json);
    }

    let res: HttpDataRes = HttpDataRes {
        status: 200,
        length: 10,
        schema: None,
        record: None,
        result: Some(valuefy(record_json_array)),
    };

    // Done
    jsonify(res)
}

#[get("/data/schema/<record>")]
pub fn data_select_one(record: String) {
    //
}
