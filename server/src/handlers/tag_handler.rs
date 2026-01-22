use axum::extract::Path;
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

use crate::auth::session::UserSession;
use crate::error::AppError;
use crate::state::AppState;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTagRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTagRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TagResponse {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
    pub created_at: String,
}

impl From<entity::tags::Model> for TagResponse {
    fn from(tag: entity::tags::Model) -> Self {
        Self {
            id: tag.id,
            name: tag.name,
            color: tag.color,
            created_at: tag.created_at.to_rfc3339(),
        }
    }
}

pub struct TagHandler;

impl TagHandler {
    async fn get_user_id(session: &Session) -> Result<i32, AppError> {
        UserSession::get(session)
            .await?
            .ok_or(AppError::Unauthorized)
    }

    pub async fn list(
        State(state): State<AppState>,
        session: Session,
    ) -> Result<Json<Vec<TagResponse>>, AppError> {
        let user_id = Self::get_user_id(&session).await?;

        let tags = entity::tags::Entity::find()
            .filter(entity::tags::Column::UserId.eq(user_id))
            .all(state.db.as_ref())
            .await?;

        Ok(Json(tags.into_iter().map(TagResponse::from).collect()))
    }

    pub async fn create(
        State(state): State<AppState>,
        session: Session,
        Json(req): Json<CreateTagRequest>,
    ) -> Result<Json<TagResponse>, AppError> {
        req.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let user_id = Self::get_user_id(&session).await?;

        let existing = entity::tags::Entity::find()
            .filter(entity::tags::Column::UserId.eq(user_id))
            .filter(entity::tags::Column::Name.eq(&req.name))
            .one(state.db.as_ref())
            .await?;

        if existing.is_some() {
            return Err(AppError::Conflict("Tag name already exists".to_string()));
        }

        let tag = entity::tags::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            name: Set(req.name),
            color: Set(req.color),
            created_at: Set(Utc::now().fixed_offset()),
        };

        let tag = tag.insert(state.db.as_ref()).await?;
        Ok(Json(TagResponse::from(tag)))
    }

    pub async fn update(
        State(state): State<AppState>,
        session: Session,
        Path(id): Path<i32>,
        Json(req): Json<UpdateTagRequest>,
    ) -> Result<Json<TagResponse>, AppError> {
        req.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let user_id = Self::get_user_id(&session).await?;

        let tag = entity::tags::Entity::find_by_id(id)
            .filter(entity::tags::Column::UserId.eq(user_id))
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;

        let mut active: entity::tags::ActiveModel = tag.into();

        if let Some(name) = req.name {
            active.name = Set(name);
        }
        if let Some(color) = req.color {
            active.color = Set(Some(color));
        }

        let tag = active.update(state.db.as_ref()).await?;
        Ok(Json(TagResponse::from(tag)))
    }

    pub async fn delete(
        State(state): State<AppState>,
        session: Session,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        let user_id = Self::get_user_id(&session).await?;

        let result = entity::tags::Entity::delete_many()
            .filter(entity::tags::Column::Id.eq(id))
            .filter(entity::tags::Column::UserId.eq(user_id))
            .exec(state.db.as_ref())
            .await?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Tag not found".to_string()));
        }

        Ok(Json(serde_json::json!({"message": "Tag deleted"})))
    }
}