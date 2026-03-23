use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct PostgresService {
    database: DatabaseConnection,
}

impl PostgresService {
    pub async fn new(db_url: &str) -> Self {
        let database: DatabaseConnection = Database::connect(db_url)
            .await
            .expect("Failed to connect to the database");

        Self { database }
    }

    pub fn connection(&self) -> DatabaseConnection {
        self.database.clone()
    }
}
