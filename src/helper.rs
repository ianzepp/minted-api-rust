use serde::Serialize;
use rocket::serde::json::{to_value, to_string, Value};

pub fn jsonify<T: Serialize>(value: T) -> String {
    match to_string(&value) {
        Ok(v) => v,
        Err(_) => "".to_string(),
    }
}

pub fn valuefy<T: Serialize>(value: T) -> Value {
    match to_value(value) {
        Ok(v) => v,
        Err(_) => Value::Null,
    }
}