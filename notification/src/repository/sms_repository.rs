use std::collections::HashMap;

use crate::models::types::SmsTemplate;

pub fn get_by_template_code(template_code: &str, db_connection: &String) -> Option<SmsTemplate> {
    println!("Get by template code: {}", template_code);
    Some(SmsTemplate {
        template_code: String::from(template_code),
        template_params: HashMap::new(),
    })
}