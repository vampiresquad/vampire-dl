use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::error::Error;

/// ডাটাবেসের সাথে কানেকশন তৈরি করার ফাংশন
pub async fn init_db(db_url: &str) -> Result<SqlitePool, Box<dyn Error>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    // ডাটাবেসে টেবিলগুলো তৈরি করা
    create_tables(&pool).await?;

    Ok(pool)
}

/// ডাউনলোড টাস্ক এবং সেগমেন্ট সেভ করার জন্য টেবিল তৈরি
async fn create_tables(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    // Tasks Table: মূল ফাইলের তথ্য রাখার জন্য
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            url TEXT NOT NULL,
            file_name TEXT NOT NULL,
            save_path TEXT NOT NULL,
            total_size INTEGER NOT NULL,
            downloaded_size INTEGER NOT NULL,
            status TEXT NOT NULL,
            created_at INTEGER NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Segments Table: ফাইলের খণ্ডগুলোর (chunks) তথ্য রাখার জন্য
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS segments (
            id INTEGER NOT NULL,
            task_id TEXT NOT NULL,
            start_byte INTEGER NOT NULL,
            end_byte INTEGER NOT NULL,
            downloaded_bytes INTEGER NOT NULL,
            status TEXT NOT NULL,
            PRIMARY KEY (id, task_id),
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
