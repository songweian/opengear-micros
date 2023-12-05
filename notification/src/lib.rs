use rocket::{Build, Rocket};
use rocket_db_pools::Database;

use crate::routes::hello_world;

pub(crate) mod configuration;
pub(crate) mod models;
pub(crate) mod remote;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;

pub fn bootstrap(mut b: Rocket<Build>) -> Rocket<Build> {
    hello_world::index();
    let routes = routes::config();
    for (k, v) in routes {
        b = b.mount(k, v);
    }
    let b = b.attach(configuration::DBPool::init());
    b
}
