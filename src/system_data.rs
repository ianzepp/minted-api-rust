use crate::models::record::Record;

pub fn select_any(schema: String) -> Vec<Record> {
    let mut records = Vec::new();

    for _ in 0..5 {
        records.push(Record::hashify(
            "7c533d2d-bdb1-4432-ae6b-21805f9bc505",
            schema.as_str()
        ));
    }

    records    
}

pub fn select_one(schema: String, record: String) -> Record {
    Record::hashify(record, schema)
}