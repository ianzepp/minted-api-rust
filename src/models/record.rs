use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rocket::serde::json::Value;

/** Custom type specifications */
pub type RecordHash = HashMap<String, Value>;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct RecordJson {
    pub meta: RecordHash,
    pub data: RecordHash,
    pub info: RecordHash,
    pub acls: RecordHash,
}
