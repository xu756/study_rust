use sqlx::{query, SqlitePool};
use sqlx::Row;

// 连接数据库
pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:./db.sqlite3").await?;
    Ok(pool)
}

// 初始化数据库
pub async fn init_db() -> Result<(), sqlx::Error> {
    let pool = connect().await?;
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS study (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            create_time TEXT NOT NULL
        )
        "#,
    )
        .execute(&pool)
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_connect() {
    let pool = connect().await.unwrap();
    let row = sqlx::query("SELECT * FROM study")
        .fetch_one(&pool)
        .await
        .unwrap();

    // Access the columns from the query result using get method
    let id: i32 = row.get("id");
    let title: String = row.get("title");
    let content: String = row.get("content");
    let create_time: String = row.get("create_time");

    // Print the results
    println!("id: {}, title: {}, content: {}, create_time: {}", id, title, content, create_time);
}
