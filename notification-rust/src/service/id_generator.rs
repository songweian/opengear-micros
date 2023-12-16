use crate::models::types::TemplateType;

pub fn gen_task_id<'a>(template_type: TemplateType) -> &'a str {
    match template_type {
        TemplateType::SmsTemplate => "sms",
        TemplateType::FeishuTemplate => "feishubot",
        _ => "unknown",
    }
}

pub fn gen_batch_id<'a>(template_type: TemplateType) -> &'a str {
    match template_type {
        TemplateType::SmsTemplate => "sms",
        TemplateType::FeishuTemplate => "feishubot",
        _ => "unknown",
    }
}
