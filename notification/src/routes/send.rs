use std::collections::HashMap;

use rocket::serde::json::Json;
use rocket::*;
use tokio::task::spawn_blocking;

use crate::configuration::DBPool;
use crate::models::types::{BatchTaskInfo, SendTaskRequest};
use crate::service::CompositedSender;

#[post("/send", data = "<task>")]
pub async fn single_template_send<'a>(
    task: Json<SendTaskRequest<'_>>,
    mut p: &DBPool,
) -> Json<BatchTaskInfo<'a>> {
    println!("Task batch_id: {}", task.batch_id);
    println!("Task template code: {}", task.template_code);
    for (k, v) in task.template_params.iter() {
        println!("Task template params: {} {}", k, v);
    }
    let composited_sender = CompositedSender {
        db: p,
        typed_sender_map: HashMap::new(),
    };

    let x = composited_sender.send(task).await;
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
