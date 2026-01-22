use axum::extract::State;
use axum::Json;
use chrono::Utc;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use serde::Deserialize;
use serde::Serialize;
use tower_sessions::Session;
use validator::Validate;

use crate::auth::password::Password;
use crate::auth::session::UserSession;
use crate::error::AppError;
use crate::state::AppState;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 100))]
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    pub username: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub message: String,
}

pub struct AuthHandler;

impl AuthHandler {
    pub async fn register(
        State(state): State<AppState>,
        session: Session,
        Json(req): Json<RegisterRequest>,
    ) -> Result<Json<AuthResponse>, AppError> {
        req.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let existing = entity::users::Entity::find()
            .filter(
                entity::users::Column::Username
                    .eq(&req.username)
                    .or(entity::users::Column::Email.eq(&req.email)),
            )
            .one(state.db.as_ref())
            .await?;

        if existing.is_some() {
            return Err(AppError::Conflict(
                "Username or email already exists".to_string(),
            ));
        }

        let now = Utc::now().fixed_offset();
        let password_hash = Password::hash(&req.password)?;

        let user = entity::users::ActiveModel {
            id: NotSet,
            username: Set(req.username),
            email: Set(req.email),
            password_hash: Set(password_hash),
            display_name: Set(req.display_name),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let user = user.insert(state.db.as_ref()).await?;
        UserSession::create(&session, user.id).await?;

        Ok(Json(AuthResponse {
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
                display_name: user.display_name,
            },
            message: "Registration successful".to_string(),
        }))
    }

    pub async fn login(
        State(state): State<AppState>,
        session: Session,
        Json(req): Json<LoginRequest>,
    ) -> Result<Json<AuthResponse>, AppError> {
        req.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let user = entity::users::Entity::find()
            .filter(entity::users::Column::Username.eq(&req.username))
            .one(state.db.as_ref())
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        if !Password::verify(&req.password, &user.password_hash)? {
            return Err(AppError::InvalidCredentials);
        }

        UserSession::create(&session, user.id).await?;

        Ok(Json(AuthResponse {
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
                display_name: user.display_name,
            },
            message: "Login successful".to_string(),
        }))
    }

    pub async fn logout(session: Session) -> Result<Json<serde_json::Value>, AppError> {
        UserSession::destroy(&session).await?;
        Ok(Json(serde_json::json!({"message": "Logout successful"})))
    }

    pub async fn me(
        State(state): State<AppState>,
        session: Session,
    ) -> Result<Json<UserResponse>, AppError> {
        let user_id = UserSession::get(&session)
            .await?
            .ok_or(AppError::Unauthorized)?;

        let user = entity::users::Entity::find_by_id(user_id)
            .one(state.db.as_ref())
            .await?
            .ok_or(AppError::Unauthorized)?;

        Ok(Json(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            display_name: user.display_name,
        }))
    }
}