use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};

// Rust definition
#[derive(Debug, Default)]
#[derive(Queryable, Selectable)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::models::schema::schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SchemaType {
    pub id: uuid::Uuid,
    pub ns: Option<String>,
    pub sc: Option<String>,
    pub schema_name: String,
    pub description: Option<String>,
}

// Table definition
diesel::table! {
    schema (id) {
        id -> diesel::sql_types::Uuid,
        ns -> Nullable<Varchar>,
        sc -> Nullable<Varchar>,
        schema_name -> Varchar,
        description -> Nullable<Text>,
    }
}