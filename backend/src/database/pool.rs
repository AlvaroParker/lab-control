use async_sqlx_session::PostgresSessionStore;

use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect_db(database_uri: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_uri).await
}

use std::process::exit;

#[derive(Clone)]
pub struct Pool {
    // Database connection
    db: DatabaseConnection,
    // Session store for admins/users
    store: PostgresSessionStore,
}

impl Pool {
    pub async fn new(db: DatabaseConnection) -> Self {
        let store = start_store().await.unwrap();

        Self { db, store }
    }
    pub async fn new_from_uri(database_uri: &str) -> Self {
        let db = match connect_db(database_uri).await {
            Ok(db) => db,
            Err(e) => {
                tracing::error!("Failed to connect to database: {}", e);
                exit(1);
            }
        };
        let store = start_store().await.unwrap();
        Self { db, store }
    }
    pub fn get_db(&self) -> &DatabaseConnection {
        &self.db
    }
    pub fn get_store(&self) -> PostgresSessionStore {
        self.store.clone()
    }
}

async fn start_store() -> Result<PostgresSessionStore, Box<dyn std::error::Error>> {
    // Creates a new sql postgres session store
    let store = PostgresSessionStore::new(&crate::DATABASE_URL.to_string()).await?;
    // Migrates the session store (creates a new table if it doesn't exist one already)
    store.migrate().await?;
    Ok(store)
}

pub async fn create_pool() -> Pool {
    let database_uri = crate::DATABASE_URL.to_string();
    Pool::new_from_uri(&database_uri).await
}
