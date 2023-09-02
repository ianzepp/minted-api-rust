use diesel::prelude::*;

// Rust definition
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::models::column::column)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ColumnType {
    pub id: uuid::Uuid,
    pub ns: Option<String>,
    pub sc: Option<String>,
    pub schema_name: String,
    pub column_name: String,
    pub description: Option<String>,
}

// Table definition
diesel::table! {
    column (id) {
        id -> diesel::sql_types::Uuid,
        ns -> Nullable<Varchar>,
        sc -> Nullable<Varchar>,
        schema_name -> Varchar,
        column_name -> Varchar,
        description -> Nullable<Text>,
    }
}