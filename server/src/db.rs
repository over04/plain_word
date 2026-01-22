use sea_orm::Database;
use sea_orm::DatabaseConnection;

use crate::config::Config;
use crate::error::AppError;

pub struct DbPool;

impl DbPool {
    pub async fn connect(config: &Config) -> Result<DatabaseConnection, AppError> {
        Database::connect(&config.database_url)
            .await
            .map_err(AppError::Database)
    }
}