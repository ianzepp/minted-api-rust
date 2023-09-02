use diesel::prelude::*;

// Rust definition
#[derive(Queryable, Selectable)]
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