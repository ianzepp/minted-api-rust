use rocket::patch;

#[patch("/data/<_schema>")]
pub fn run(_schema: String) {
    // TODO
}