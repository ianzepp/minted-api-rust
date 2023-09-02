use rocket::patch;

#[patch("/data/<_schema>/<_record>")]
pub fn run(_schema: String, _record: String) {
    // TODO
}