use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};

/// Connect sqlite and initialize required tables.
pub async fn connect_database(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(database_url).await?;
    init_schema(&db).await?;
    Ok(db)
}

async fn init_schema(db: &DatabaseConnection) -> Result<(), DbErr> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            created_at INTEGER NOT NULL
        )
    "#;
    let backend = db.get_database_backend();
    db.execute(Statement::from_string(backend, sql.to_string()))
        .await?;
    Ok(())
}
