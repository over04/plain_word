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
pub struct CreateChapterRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateChapterRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ChapterResponse {
    pub id: i32,
    pub wordbook_id: i32,
    pub name: String,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl From<entity::chapters::Model> for ChapterResponse {
    fn from(ch: entity::chapters::Model) -> Self {
        Self {
            id: ch.id,
            wordbook_id: ch.wordbook_id,
            name: ch.name,
            sort_order: ch.sort_order,
            created_at: ch.created_at.to_rfc3339(),
            updated_at: ch.updated_at.to_rfc3339(),
        }
    }
}

pub struct ChapterHandler;

impl ChapterHandler {
    async fn verify_wordbook_ownership(
        state: &AppState,
        session: &Session,
        wordbook_id: i32,
    ) -> Result<(), AppError> {
        let user_id = UserSession::get(session)
            .await?
            .ok_or(AppError::Unauthorized)?;

        entity::wordbooks::Entity::find_by_id(wordbook_id)
            .filter(entity::wordbooks::Column::UserId.eq(user_id))
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Wordbook not found".to_string()))?;

        Ok(())
    }

    pub async fn list(
        State(state): State<AppState>,
        session: Session,
        Path(wordbook_id): Path<i32>,
    ) -> Result<Json<Vec<ChapterResponse>>, AppError> {
        Self::verify_wordbook_ownership(&state, &session, wordbook_id).await?;

        let chapters = entity::chapters::Entity::find()
            .filter(entity::chapters::Column::WordbookId.eq(wordbook_id))
            .order_by_asc(entity::chapters::Column::SortOrder)
            .all(state.db.as_ref())
            .await?;

        Ok(Json(
            chapters.into_iter().map(ChapterResponse::from).collect(),
        ))
    }

    pub async fn create(
        State(state): State<AppState>,
        session: Session,
        Path(wordbook_id): Path<i32>,
        Json(req): Json<CreateChapterRequest>,
    ) -> Result<Json<ChapterResponse>, AppError> {
        req.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        Self::verify_wordbook_ownership(&state, &session, wordbook_id).await?;

        let now = Utc::now().fixed_offset();

        let max_order = entity::chapters::Entity::find()
            .filter(entity::chapters::Column::WordbookId.eq(wordbook_id))
            .order_by_desc(entity::chapters::Column::SortOrder)
            .one(state.db.as_ref())
            .await?
            .map(|c| c.sort_order + 1)
            .unwrap_or(0);

        let chapter = entity::chapters::ActiveModel {
            id: NotSet,
            wordbook_id: Set(wordbook_id),
            name: Set(req.name),
            sort_order: Set(max_order),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let chapter = chapter.insert(state.db.as_ref()).await?;
        Ok(Json(ChapterResponse::from(chapter)))
    }

    pub async fn update(
        State(state): State<AppState>,
        session: Session,
        Path((wordbook_id, chapter_id)): Path<(i32, i32)>,
        Json(req): Json<UpdateChapterRequest>,
    ) -> Result<Json<ChapterResponse>, AppError> {
        req.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        Self::verify_wordbook_ownership(&state, &session, wordbook_id).await?;

        let chapter = entity::chapters::Entity::find_by_id(chapter_id)
            .filter(entity::chapters::Column::WordbookId.eq(wordbook_id))
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Chapter not found".to_string()))?;

        let mut active: entity::chapters::ActiveModel = chapter.into();
        active.updated_at = Set(Utc::now().fixed_offset());

        if let Some(name) = req.name {
            active.name = Set(name);
        }
        if let Some(sort_order) = req.sort_order {
            active.sort_order = Set(sort_order);
        }

        let chapter = active.update(state.db.as_ref()).await?;
        Ok(Json(ChapterResponse::from(chapter)))
    }

    pub async fn delete(
        State(state): State<AppState>,
        session: Session,
        Path((wordbook_id, chapter_id)): Path<(i32, i32)>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Self::verify_wordbook_ownership(&state, &session, wordbook_id).await?;

        let result = entity::chapters::Entity::delete_many()
            .filter(entity::chapters::Column::Id.eq(chapter_id))
            .filter(entity::chapters::Column::WordbookId.eq(wordbook_id))
            .exec(state.db.as_ref())
            .await?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Chapter not found".to_string()));
        }

        Ok(Json(serde_json::json!({"message": "Chapter deleted"})))
    }
}