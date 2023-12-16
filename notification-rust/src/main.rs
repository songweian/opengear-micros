#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let r = rocket::build();
    let r = notification::bootstrap(r);
    r
}
