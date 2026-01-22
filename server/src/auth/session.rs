use serde::Deserialize;
use serde::Serialize;
use tower_sessions::Session;

use crate::error::AppError;

const USER_ID_KEY: &str = "user_id";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: i32,
}

impl UserSession {
    pub async fn create(session: &Session, user_id: i32) -> Result<(), AppError> {
        session
            .insert(USER_ID_KEY, user_id)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))
    }

    pub async fn get(session: &Session) -> Result<Option<i32>, AppError> {
        session
            .get::<i32>(USER_ID_KEY)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))
    }

    pub async fn destroy(session: &Session) -> Result<(), AppError> {
        session.flush().await.map_err(|e| AppError::Internal(e.to_string()))
    }
}