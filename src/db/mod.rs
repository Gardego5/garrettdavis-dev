use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use crate::config::Config;

pub async fn get_connection(Config { db_url, .. }: &Config) -> anyhow::Result<SqlitePool> {
    println!("DB_URL: {db_url}");

    if Sqlite::database_exists(db_url).await? {
        println!("Database already exists");
    } else {
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Database created"),
            Err(e) => panic!("Error creating database: {}", e),
        }
    }

    let db = SqlitePool::connect(db_url).await?;

    sqlx::migrate!().run(&db).await?;

    Ok(db)
}
