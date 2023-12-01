use std::collections::HashMap;
use std::error::Error;

use rocket::serde::json::Json;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SendTaskRequest<'r> {
    pub batch_id: &'r str,
    pub template_code: &'r str,
    pub template_params: HashMap<&'r str, &'r str>,
}

pub struct InnerSendTask<'a> {
    pub batch_id: &'a str,
    pub template_type: TemplateType,
    pub template_params: HashMap<&'a str, &'a str>,
}

pub struct SendTaskId<'a> {
    pub target: &'a str,
    pub task_id: &'a str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BatchTaskInfo<'a> {
    pub batch_id: &'a str,
    pub task_id_map: HashMap<&'a str, &'a str>,
}

impl InnerSendTask<'_> {
    pub(crate) fn from_send_task_request<'a>(p0: &'a Json<SendTaskRequest>) -> Result<InnerSendTask<'a>, Box<dyn Error>> {
        let batch_id = p0.batch_id;
        let template_type = TemplateType::from_template_code(&p0.template_code);
        let mut template_params = HashMap::new();
        for (k, v) in p0.template_params.iter() {
            template_params.insert(*k, *v);
        }

        Ok(InnerSendTask {
            batch_id,
            template_type,
            template_params,
        })
    }
}

pub struct SmsTemplate {
    pub template_code: String,
    pub template_params: HashMap<String, String>,
}

pub enum TemplateType {
    SmsTemplate,
    EmailTemplate,
    FeishuTemplate,
}

impl TemplateType {
    pub(crate) fn from_template_code(p0: &&str) -> TemplateType {
        TemplateType::SmsTemplate
    }
}