use diesel::prelude::*;

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