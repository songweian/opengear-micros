use std::collections::HashMap;
use rocket::{Build, Rocket};
use rocket_db_pools::Database;
use crate::configuration::DBPool;

use crate::routes::hello_world;
use crate::service::{CompositedSender, SmsSender};

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
    bootstrap_service(&b);
    b
}

pub fn bootstrap_service(b: &Rocket<Build>) {
    let db_pool = DBPool::fetch(&b).unwrap();

    let sms_sender = SmsSender::new(&db_pool);

    let mut typed_sender_map = HashMap::new();
    typed_sender_map.insert(crate::models::types::TemplateType::SmsTemplate, Box::new(sms_sender));

    let composited_sender = CompositedSender {
        db: &db_pool,
        typed_sender_map: HashMap::new(),
    };


}
