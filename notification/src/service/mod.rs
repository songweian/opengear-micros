use std::error::Error;

pub use composited_sender::CompositedSender;
pub use feishubot::FeishuBotSender;
pub use sms::sms_service::SmsSender;

use crate::models::types::InnerSendTask;

mod composited_sender;

mod sms;

mod feishubot;

mod id_generator;

pub trait SendMessage {
    fn send<'a>(send_task: InnerSendTask) -> Result<(), &'a str> where Self: Sized;
}

pub trait QuerySendStatus {
    fn query_send_status(task_id: &str) -> Result<(), Box<dyn Error>>;
}

pub trait QuerySendTask {
    fn query_send_task(task_id: &str) -> Result<(), Box<dyn Error>>;

    fn query_send_task_by_batch_id(batch_id: &str) -> Result<(), Box<dyn Error>>;
}

pub enum SendStatus {
    Success,
    Fail,
    Sending,
    Unknown,
}