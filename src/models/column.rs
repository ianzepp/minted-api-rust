use diesel::prelude::*;
use diesel::sql_types::Uuid;

// Rust definition
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::models::column::column)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ColumnType {
    pub id: Uuid,
    pub ns: Option<String>,
    pub sc: Option<String>,
    pub schema_name: String,
    pub column_name: String,
    pub description: Option<String>,
}

// Table definition
diesel::table! {
    column (id) {
        id -> Uuid,
        ns -> Nullable<Varchar>,
        sc -> Nullable<Varchar>,
        schema_name -> Varchar,
        column_name -> Varchar,
        description -> Nullable<Text>,
    }
}