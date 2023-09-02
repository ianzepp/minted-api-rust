use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rocket::serde::json::Value;

/** Custom type specifications */
pub type RecordHash = HashMap<String, Value>;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Record {
    pub data: RecordHash,
    pub meta: RecordHash,
}

pub fn to_record(flat: RecordHash) -> Record {
    let mut data = flat.clone();
    data.retain(|k, _| k != "id" && k != "ns" && k != "sc");

    let mut meta = RecordHash::new();
    meta.insert("id".to_string(), flat["id"].clone());
    meta.insert("ns".to_string(), flat["ns"].clone());
    meta.insert("sc".to_string(), flat["sc"].clone());

    Record {
        data: data,
        meta: meta,
    }
}

pub fn to_record_flat(record: Record) -> HashMap<String, Value> {
    let mut flat = RecordHash::new();

    flat.extend(record.data.clone());
    flat.extend(record.meta.clone());

    // Done
    flat
}

