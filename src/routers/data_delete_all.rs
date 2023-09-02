use rocket::delete;

#[delete("/data/<_schema>")]
pub fn run(_schema: String) {
    // TODO
}