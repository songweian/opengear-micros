use std::collections::HashMap;

use rocket::serde::json::Json;

use crate::configuration::DBPool;
use crate::models::types::{InnerSendTask, SendTaskRequest, TemplateType};
use crate::repository::TemplateRepository;
use crate::service::{SendMessage, SmsSender};

pub struct CompositedSender {
    pub db: &'static DBPool,
    pub typed_sender_map: HashMap<TemplateType, Box<dyn SendMessage>>,
}

unsafe impl Send for CompositedSender {}

unsafe impl Sync for CompositedSender {}

impl CompositedSender {
    pub fn singleton(db: &DBPool, typed_sender_map: HashMap<TemplateType, Box<dyn SendMessage>>) -> Self {
        CompositedSender {
            db,
            typed_sender_map,
        }
    }
    pub async fn send(&self, send_task_request: Json<SendTaskRequest<'_>>) {
        println!("send service task");
        let template_code = send_task_request.template_code;
        let template_type = TemplateType::from_template_code(&template_code);

        let template_repository = TemplateRepository::from_connection(&self.db);
        let template = template_repository
            .get_template_content(&template_code)
            .await
            .expect("get template error");
        println!("template content: {}", template);

        let inner_send_task = InnerSendTask::from_send_task_request(&send_task_request).unwrap();
        match template_type {
            TemplateType::SmsTemplate => {
                println!("send sms");
                SmsSender::send(inner_send_task).expect("send sms error");
            }
            TemplateType::EmailTemplate => {
                println!("send email");
            }
            _ => {
                println!("send other");
            }
        }
    }
}
