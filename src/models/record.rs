use std::collections::HashMap;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Value;

/** Custom type specifications */
pub type RecordHash = HashMap<String, Value>;

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Record {
    pub meta: RecordHash,
    pub data: RecordHash,
}

impl Record {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hashify<S: Into<String>>(id: S, name: S) -> Self {
        let mut data = RecordHash::new();
        data.insert("id".to_string(), Value::String(id.into()));
        data.insert("ns".to_string(), Value::Null);

        let mut meta = RecordHash::new();
        meta.insert("sc".to_string(), Value::Null);
        meta.insert("schema_name".to_string(), Value::String(name.into()));
        meta.insert("description".to_string(), Value::Null);

        Self { data, meta }
    }

    pub fn flatten(self) -> RecordHash {
        let mut flat = RecordHash::new();

        flat.extend(self.data);
        flat.extend(self.meta);

        flat
    }

    pub fn broaden(flat: RecordHash) -> Self {
        let mut data = flat.clone();
        data.retain(|k, _| k != "id" && k != "ns" && k != "sc");

        let mut meta = flat.clone();
        meta.retain(|k, _| k == "id" || k == "ns" || k == "sc");

        Self { data, meta }
    }
}
