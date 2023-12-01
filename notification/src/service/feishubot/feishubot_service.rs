use std::error::Error;

use crate::models::types::InnerSendTask;
use crate::service::{QuerySendStatus, QuerySendTask, SendMessage};

pub struct FeishuBotSender {
    pub db: String,
}

impl FeishuBotSender {
    pub fn new() -> Self {
        FeishuBotSender {
            db: String::from("feishu_bot"),
        }
    }
}

impl SendMessage for FeishuBotSender {
    fn send<'a>(send_task: InnerSendTask) -> Result<(), &'a str> {
        Ok(())
    }
}

impl QuerySendStatus for FeishuBotSender {
    fn query_send_status(task_id: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}


impl QuerySendTask for FeishuBotSender {
    fn query_send_task(task_id: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn query_send_task_by_batch_id(batch_id: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}