use rocket::get;

#[get("/data/<_schema>")]
pub fn run(_schema: String) {
    // TODO
}