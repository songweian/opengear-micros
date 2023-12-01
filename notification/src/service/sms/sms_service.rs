use std::collections::HashMap;

use crate::models::types::InnerSendTask;
use crate::service::SendMessage;

pub struct SmsSender {
    pub db: String,
}

impl SmsSender {
    pub fn new() -> Self {
        SmsSender {
            db: String::from("sms"),
        }
    }
}

impl SendMessage for SmsSender {
    fn send<'a>(send_task: InnerSendTask) -> Result<(), &'a str> {
        Ok(())
    }
}

pub fn send(template_code: &str, template_params: HashMap<&str, &str>) {
    println!("Send template code: {}", template_code);
    for (k, v) in template_params.iter() {
        println!("Send template params: {} {}", k, v);
    }

    let sms_template = crate::models::types::SmsTemplate {
        template_code: String::from(template_code),
        template_params: template_params.iter().map(|(k, v)| (String::from(*k), String::from(*v))).collect(),
    };
    inner_send(&sms_template.template_code, sms_template.template_params);
}

fn inner_send(template_code: &str, template_params: HashMap<String, String>) {
    println!("Inner send template code: {}", template_code);
    for (k, v) in template_params.iter() {
        println!("Inner send template params: {} {}", k, v);
    }
}
