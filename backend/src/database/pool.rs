use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect_db(database_uri: &str) -> Result<DatabaseConnection, DbErr> {
    Ok(Database::connect(database_uri).await?)
}

use std::process::exit;

use dotenvy::dotenv;
use dotenvy_macro::dotenv;

#[derive(Clone)]
pub struct Pool {
    db: DatabaseConnection,
}

impl Pool {
    pub async fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
    pub async fn new_from_uri(database_uri: &str) -> Self {
        let db = match connect_db(database_uri).await {
            Ok(db) => db,
            Err(e) => {
                eprintln!("Failed to connect to database: {}", e);
                exit(1);
            }
        };
        Self { db }
    }
    pub fn get_db(&self) -> &DatabaseConnection {
        &self.db
    }
}

pub async fn create_pool() -> Pool {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");

    Pool::new_from_uri(database_uri).await
}
