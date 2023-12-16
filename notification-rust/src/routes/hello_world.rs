use rocket::*;

#[get("/")]
pub fn index() -> String {
    String::from("hello world")
}
