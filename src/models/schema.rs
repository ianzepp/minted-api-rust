use diesel::prelude::*;
use diesel::pg::sql_types::Uuid;

// Rust definition
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Schema {
    pub id: Uuid,
    pub ns: Option<String>,
    pub sc: Option<String>,
    pub schema_name: String,
    pub description: Option<String>,
}

// Table definition
diesel::table! {
    schema (id) {
        id -> Uuid,
        ns -> Nullable<Varchar>,
        sc -> Nullable<Varchar>,
        schema_name -> Varchar,
        description -> Nullable<Text>,
    }
}