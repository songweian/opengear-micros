use rocket::routes;
use rocket::Route;

pub mod hello_world;
pub mod send;

pub fn config() -> Vec<(&'static str, Vec<Route>)> {
    let mut routes = Vec::new();
    routes.push(("/", routes![hello_world::index]));
    routes.push(("/", routes![send::single_template_send]));
    routes
}
