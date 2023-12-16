use rocket::Rocket;
use rocket_db_pools::sqlx::{self, Acquire, Row};
use rocket_db_pools::Database;

#[derive(Database)]
#[database("sqlite_logs")]
pub struct DBPool(sqlx::SqlitePool);

async fn test() {
    let x = Rocket::build();
    let xb = DBPool::fetch(&x);

    let mut co = match xb.unwrap().try_acquire() {
        None => {
            println!("none");
            return;
        }
        Some(sqlx) => sqlx,
    };
    let xoo = co.acquire().await.unwrap();
    let row = sqlx::query("SELECT content FROM template WHERE code = ?")
        .bind("test")
        .fetch_one(xoo)
        .await
        .unwrap();
    let content: String = row.try_get(0).unwrap();
    println!("content: {}", content);
}
