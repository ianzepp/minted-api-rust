use rocket::get;

#[get("/data/<_schema>/<_record>")]
pub fn run(_schema: String, _record: String) {
    // TODO
}