use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use chrono::Utc;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::Set;
use serde::Deserialize;
use serde::Serialize;
use tower_sessions::Session;
use validator::Validate;

use crate::auth::session::UserSession;
use crate::error::AppError;
use crate::state::AppState;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateWordbookRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateWordbookRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct WordbookResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl From<entity::wordbooks::Model> for WordbookResponse {
    fn from(wb: entity::wordbooks::Model) -> Self {
        Self {
            id: wb.id,
            name: wb.name,
            description: wb.description,
            cover_url: wb.cover_url,
            sort_order: wb.sort_order,
            created_at: wb.created_at.to_rfc3339(),
            updated_at: wb.updated_at.to_rfc3339(),
        }
    }
}

pub struct WordbookHandler;

impl WordbookHandler {
    async fn get_user_id(session: &Session) -> Result<i32, AppError> {
        UserSession::get(session)
            .await?
            .ok_or(AppError::Unauthorized)
    }

    pub async fn list(
        State(state): State<AppState>,
        session: Session,
    ) -> Result<Json<Vec<WordbookResponse>>, AppError> {
        let user_id = Self::get_user_id(&session).await?;

        let wordbooks = entity::wordbooks::Entity::find()
            .filter(entity::wordbooks::Column::UserId.eq(user_id))
            .order_by_asc(entity::wordbooks::Column::SortOrder)
            .all(state.db.as_ref())
            .await?;

        Ok(Json(
            wordbooks.into_iter().map(WordbookResponse::from).collect(),
        ))
    }

    pub async fn get(
        State(state): State<AppState>,
        session: Session,
        Path(id): Path<i32>,
    ) -> Result<Json<WordbookResponse>, AppError> {
        let user_id = Self::get_user_id(&session).await?;

        let wordbook = entity::wordbooks::Entity::find_by_id(id)
            .filter(entity::wordbooks::Column::UserId.eq(user_id))
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Wordbook not found".to_string()))?;

        Ok(Json(WordbookResponse::from(wordbook)))
    }

    pub async fn create(
        State(state): State<AppState>,
        session: Session,
        Json(req): Json<CreateWordbookRequest>,
    ) -> Result<Json<WordbookResponse>, AppError> {
        req.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let user_id = Self::get_user_id(&session).await?;
        let now = Utc::now().fixed_offset();

        let max_order = entity::wordbooks::Entity::find()
            .filter(entity::wordbooks::Column::UserId.eq(user_id))
            .order_by_desc(entity::wordbooks::Column::SortOrder)
            .one(state.db.as_ref())
            .await?
            .map(|w| w.sort_order + 1)
            .unwrap_or(0);

        let wordbook = entity::wordbooks::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            name: Set(req.name),
            description: Set(req.description),
            cover_url: Set(req.cover_url),
            sort_order: Set(max_order),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let wordbook = wordbook.insert(state.db.as_ref()).await?;
        Ok(Json(WordbookResponse::from(wordbook)))
    }

    pub async fn update(
        State(state): State<AppState>,
        session: Session,
        Path(id): Path<i32>,
        Json(req): Json<UpdateWordbookRequest>,
    ) -> Result<Json<WordbookResponse>, AppError> {
        req.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let user_id = Self::get_user_id(&session).await?;

        let wordbook = entity::wordbooks::Entity::find_by_id(id)
            .filter(entity::wordbooks::Column::UserId.eq(user_id))
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Wordbook not found".to_string()))?;

        let mut active: entity::wordbooks::ActiveModel = wordbook.into();
        active.updated_at = Set(Utc::now().fixed_offset());

        if let Some(name) = req.name {
            active.name = Set(name);
        }
        if let Some(description) = req.description {
            active.description = Set(Some(description));
        }
        if let Some(cover_url) = req.cover_url {
            active.cover_url = Set(Some(cover_url));
        }
        if let Some(sort_order) = req.sort_order {
            active.sort_order = Set(sort_order);
        }

        let wordbook = active.update(state.db.as_ref()).await?;
        Ok(Json(WordbookResponse::from(wordbook)))
    }

    pub async fn delete(
        State(state): State<AppState>,
        session: Session,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        let user_id = Self::get_user_id(&session).await?;

        let result = entity::wordbooks::Entity::delete_many()
            .filter(entity::wordbooks::Column::Id.eq(id))
            .filter(entity::wordbooks::Column::UserId.eq(user_id))
            .exec(state.db.as_ref())
            .await?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Wordbook not found".to_string()));
        }

        Ok(Json(serde_json::json!({"message": "Wordbook deleted"})))
    }
}