use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::{Acquire, Row};

use crate::configuration::DBPool;

pub struct TemplateRepository<'a> {
    pub db: &'a DBPool,
}

impl TemplateRepository<'_> {
    pub fn from_connection(p0: &DBPool) -> TemplateRepository {
        TemplateRepository { db: p0 }
    }
    pub async fn get_template_content(&self, template_code: &str) -> Option<String> {
        let mut c = self.db.try_acquire().unwrap();
        let connection = c.acquire().await.unwrap();
        let row = sqlx::query("SELECT content FROM template WHERE code = ?")
            .bind(template_code)
            .fetch_one(connection)
            .await;
        match row {
            Ok(row) => {
                let content: String = row.try_get(0).unwrap();
                Some(content)
            }
            Err(err) => {
                println!("get template error {}", err);
                Some("nothing".to_string())
            }
        }
    }
}
