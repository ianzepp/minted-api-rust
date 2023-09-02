use rocket::post;

#[post("/data/<_schema>")]
pub fn run(_schema: String) {
    // TODO
}