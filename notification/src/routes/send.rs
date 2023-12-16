use std::collections::HashMap;

use rocket::serde::json::Json;
use rocket::*;
use rocket_db_pools::Connection;
use tokio::task::spawn_blocking;

use crate::configuration::DBPool;
use crate::models::types::{BatchTaskInfo, SendTaskRequest};
use crate::service::CompositedSender;

#[post("/send", data = "<task>")]
pub async fn single_template_send<'a>(
    task: Json<SendTaskRequest<'_>>,
    mut db: Connection<DBPool>,
) -> Json<BatchTaskInfo<'a>> {
    println!("Task batch_id: {}", task.batch_id);
    println!("Task template code: {}", task.template_code);
    for (k, v) in task.template_params.iter() {
        println!("Task template params: {} {}", k, v);
    }

    spawn_blocking(move || {
        let info = BatchTaskInfo {
            batch_id: "hello",
            task_id_map: HashMap::new(),
        };
        Json(info)
    })
    .await
    .unwrap()
}
