diesel::table! {
    schema (id) {
        id -> Uuid,
        ns -> Nullable<Varchar>,
        sc -> Nullable<Varchar>,
        schema_name -> Varchar,
        description -> Nullable<Text>,
    }
}