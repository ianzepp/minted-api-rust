use rocket::get;

#[get("/test/ping")]
pub fn run() -> String {
    String::from("pong");
}