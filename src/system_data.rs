use crate::models::record::Record;
use crate::models::record::RecordHash;
use crate::models::record::to_record;
use rocket::serde::json::Value;

fn generate_sample(schema: String, record: String) -> Record {
    let mut record_hash = RecordHash::new();
    record_hash.insert("id".to_string(), Value::String(record));
    record_hash.insert("ns".to_string(), Value::Null);
    record_hash.insert("sc".to_string(), Value::Null);
    record_hash.insert("schema_name".to_string(), Value::String(schema));
    record_hash.insert("description".to_string(), Value::Null);

    to_record(record_hash)
}

pub fn select_any(schema: String) -> Vec<Record> {
    let mut records = Vec::new();

    for _ in 0..5 {
        records.push(generate_sample(schema.clone(), "7c533d2d-bdb1-4432-ae6b-21805f9bc505".to_string()));
    }

    records    
}

pub fn select_one(schema: String, record: String) -> Record {
    generate_sample(schema, record)
}