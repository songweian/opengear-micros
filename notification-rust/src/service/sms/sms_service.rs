use std::collections::HashMap;
use crate::configuration::DBPool;

use crate::models::types::InnerSendTask;
use crate::service::SendMessage;

pub struct SmsSender<'a> {
    pub db: &'a DBPool,
}

impl SmsSender<'_> {
    pub fn new(db: &DBPool,) -> Self {
        SmsSender {
            db: db,
        }
    }
}

impl SendMessage for SmsSender<'_> {
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
        template_params: template_params
            .iter()
            .map(|(k, v)| (String::from(*k), String::from(*v)))
            .collect(),
    };
    inner_send(&sms_template.template_code, sms_template.template_params);
}

fn inner_send(template_code: &str, template_params: HashMap<String, String>) {
    println!("Inner send template code: {}", template_code);
    for (k, v) in template_params.iter() {
        println!("Inner send template params: {} {}", k, v);
    }
}
