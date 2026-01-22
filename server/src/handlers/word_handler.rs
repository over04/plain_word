use axum::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use chrono::Utc;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
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
pub struct CreateWordRequest {
    #[validate(length(min = 1, max = 500))]
    pub source: String,
    #[validate(length(min = 1, max = 500))]
    pub translation: String,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateWordRequest {
    #[validate(length(min = 1, max = 500))]
    pub source: Option<String>,
    #[validate(length(min = 1, max = 500))]
    pub translation: Option<String>,
    pub note: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct WordQueryParams {
    pub tag_ids: Option<String>,
    pub shuffle: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTagsRequest {
    pub tag_ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct BatchDeleteRequest {
    pub word_ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct BatchUpdateTagsRequest {
    pub word_ids: Vec<i32>,
    pub add_tag_ids: Vec<i32>,
    pub remove_tag_ids: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct BatchOperationResponse {
    pub affected: usize,
}

#[derive(Debug, Serialize)]
pub struct WordResponse {
    pub id: i32,
    pub chapter_id: i32,
    pub source: String,
    pub translation: String,
    pub note: Option<String>,
    pub sort_order: i32,
    pub tags: Vec<TagInfo>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct TagInfo {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
}

pub struct WordHandler;

impl WordHandler {
    async fn verify_chapter_ownership(
        state: &AppState,
        session: &Session,
        chapter_id: i32,
    ) -> Result<i32, AppError> {
        let user_id = UserSession::get(session)
            .await?
            .ok_or(AppError::Unauthorized)?;

        let chapter = entity::chapters::Entity::find_by_id(chapter_id)
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Chapter not found".to_string()))?;

        entity::wordbooks::Entity::find_by_id(chapter.wordbook_id)
            .filter(entity::wordbooks::Column::UserId.eq(user_id))
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Wordbook not found".to_string()))?;

        Ok(user_id)
    }

    async fn get_word_with_tags(
        state: &AppState,
        word: entity::words::Model,
    ) -> Result<WordResponse, AppError> {
        let word_tags = entity::word_tags::Entity::find()
            .filter(entity::word_tags::Column::WordId.eq(word.id))
            .all(state.db.as_ref())
            .await?;

        let tag_ids: Vec<i32> = word_tags.iter().map(|wt| wt.tag_id).collect();

        let tags = if tag_ids.is_empty() {
            vec![]
        } else {
            entity::tags::Entity::find()
                .filter(entity::tags::Column::Id.is_in(tag_ids))
                .all(state.db.as_ref())
                .await?
                .into_iter()
                .map(|t| TagInfo {
                    id: t.id,
                    name: t.name,
                    color: t.color,
                })
                .collect()
        };

        Ok(WordResponse {
            id: word.id,
            chapter_id: word.chapter_id,
            source: word.source,
            translation: word.translation,
            note: word.note,
            sort_order: word.sort_order,
            tags,
            created_at: word.created_at.to_rfc3339(),
            updated_at: word.updated_at.to_rfc3339(),
        })
    }

    pub async fn list(
        State(state): State<AppState>,
        session: Session,
        Path(chapter_id): Path<i32>,
        Query(params): Query<WordQueryParams>,
    ) -> Result<Json<Vec<WordResponse>>, AppError> {
        Self::verify_chapter_ownership(&state, &session, chapter_id).await?;

        let mut query =
            entity::words::Entity::find().filter(entity::words::Column::ChapterId.eq(chapter_id));

        if let Some(tag_ids_str) = &params.tag_ids {
            let tag_ids: Vec<i32> = tag_ids_str
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            if !tag_ids.is_empty() {
                let word_ids: Vec<i32> = entity::word_tags::Entity::find()
                    .filter(entity::word_tags::Column::TagId.is_in(tag_ids))
                    .all(state.db.as_ref())
                    .await?
                    .into_iter()
                    .map(|wt| wt.word_id)
                    .collect();

                query = query.filter(entity::words::Column::Id.is_in(word_ids));
            }
        }

        let mut words = query
            .order_by_asc(entity::words::Column::SortOrder)
            .all(state.db.as_ref())
            .await?;

        if params.shuffle.unwrap_or(false) {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let seed = Utc::now().timestamp_millis();
            words.sort_by(|a, b| {
                let mut ha = DefaultHasher::new();
                let mut hb = DefaultHasher::new();
                (a.id, seed).hash(&mut ha);
                (b.id, seed).hash(&mut hb);
                ha.finish().cmp(&hb.finish())
            });
        }

        let mut responses = Vec::with_capacity(words.len());
        for word in words {
            responses.push(Self::get_word_with_tags(&state, word).await?);
        }

        Ok(Json(responses))
    }

    pub async fn create(
        State(state): State<AppState>,
        session: Session,
        Path(chapter_id): Path<i32>,
        Json(req): Json<CreateWordRequest>,
    ) -> Result<Json<WordResponse>, AppError> {
        req.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        Self::verify_chapter_ownership(&state, &session, chapter_id).await?;

        let now = Utc::now().fixed_offset();

        let max_order = entity::words::Entity::find()
            .filter(entity::words::Column::ChapterId.eq(chapter_id))
            .order_by_desc(entity::words::Column::SortOrder)
            .one(state.db.as_ref())
            .await?
            .map(|w| w.sort_order + 1)
            .unwrap_or(0);

        let word = entity::words::ActiveModel {
            id: NotSet,
            chapter_id: Set(chapter_id),
            source: Set(req.source),
            translation: Set(req.translation),
            note: Set(req.note),
            sort_order: Set(max_order),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let word = word.insert(state.db.as_ref()).await?;
        Self::get_word_with_tags(&state, word).await.map(Json)
    }

    pub async fn update(
        State(state): State<AppState>,
        session: Session,
        Path((chapter_id, word_id)): Path<(i32, i32)>,
        Json(req): Json<UpdateWordRequest>,
    ) -> Result<Json<WordResponse>, AppError> {
        req.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        Self::verify_chapter_ownership(&state, &session, chapter_id).await?;

        let word = entity::words::Entity::find_by_id(word_id)
            .filter(entity::words::Column::ChapterId.eq(chapter_id))
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Word not found".to_string()))?;

        let mut active: entity::words::ActiveModel = word.into();
        active.updated_at = Set(Utc::now().fixed_offset());

        if let Some(source) = req.source {
            active.source = Set(source);
        }
        if let Some(translation) = req.translation {
            active.translation = Set(translation);
        }
        if let Some(note) = req.note {
            active.note = Set(Some(note));
        }
        if let Some(sort_order) = req.sort_order {
            active.sort_order = Set(sort_order);
        }

        let word = active.update(state.db.as_ref()).await?;
        Self::get_word_with_tags(&state, word).await.map(Json)
    }

    pub async fn delete(
        State(state): State<AppState>,
        session: Session,
        Path((chapter_id, word_id)): Path<(i32, i32)>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Self::verify_chapter_ownership(&state, &session, chapter_id).await?;

        let word = entity::words::Entity::find_by_id(word_id)
            .filter(entity::words::Column::ChapterId.eq(chapter_id))
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Word not found".to_string()))?;

        word.delete(state.db.as_ref()).await?;

        Ok(Json(serde_json::json!({"message": "Word deleted"})))
    }

    pub async fn update_tags(
        State(state): State<AppState>,
        session: Session,
        Path((chapter_id, word_id)): Path<(i32, i32)>,
        Json(req): Json<UpdateTagsRequest>,
    ) -> Result<Json<WordResponse>, AppError> {
        let user_id = Self::verify_chapter_ownership(&state, &session, chapter_id).await?;

        let word = entity::words::Entity::find_by_id(word_id)
            .filter(entity::words::Column::ChapterId.eq(chapter_id))
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Word not found".to_string()))?;

        for tag_id in &req.tag_ids {
            entity::tags::Entity::find_by_id(*tag_id)
                .filter(entity::tags::Column::UserId.eq(user_id))
                .one(state.db.as_ref())
                .await?
                .ok_or_else(|| AppError::NotFound(format!("Tag {} not found", tag_id)))?;
        }

        entity::word_tags::Entity::delete_many()
            .filter(entity::word_tags::Column::WordId.eq(word_id))
            .exec(state.db.as_ref())
            .await?;

        for tag_id in req.tag_ids {
            let word_tag = entity::word_tags::ActiveModel {
                word_id: Set(word_id),
                tag_id: Set(tag_id),
            };
            word_tag.insert(state.db.as_ref()).await?;
        }

        Self::get_word_with_tags(&state, word).await.map(Json)
    }

    pub async fn batch_delete(
        State(state): State<AppState>,
        session: Session,
        Path(chapter_id): Path<i32>,
        Json(req): Json<BatchDeleteRequest>,
    ) -> Result<Json<BatchOperationResponse>, AppError> {
        Self::verify_chapter_ownership(&state, &session, chapter_id).await?;

        if req.word_ids.is_empty() {
            return Ok(Json(BatchOperationResponse { affected: 0 }));
        }

        let result = entity::words::Entity::delete_many()
            .filter(entity::words::Column::ChapterId.eq(chapter_id))
            .filter(entity::words::Column::Id.is_in(req.word_ids))
            .exec(state.db.as_ref())
            .await?;

        Ok(Json(BatchOperationResponse {
            affected: result.rows_affected as usize,
        }))
    }

    pub async fn batch_update_tags(
        State(state): State<AppState>,
        session: Session,
        Path(chapter_id): Path<i32>,
        Json(req): Json<BatchUpdateTagsRequest>,
    ) -> Result<Json<BatchOperationResponse>, AppError> {
        let user_id = Self::verify_chapter_ownership(&state, &session, chapter_id).await?;

        if req.word_ids.is_empty() {
            return Ok(Json(BatchOperationResponse { affected: 0 }));
        }

        for tag_id in req.add_tag_ids.iter().chain(req.remove_tag_ids.iter()) {
            entity::tags::Entity::find_by_id(*tag_id)
                .filter(entity::tags::Column::UserId.eq(user_id))
                .one(state.db.as_ref())
                .await?
                .ok_or_else(|| AppError::NotFound(format!("Tag {} not found", tag_id)))?;
        }

        let words = entity::words::Entity::find()
            .filter(entity::words::Column::ChapterId.eq(chapter_id))
            .filter(entity::words::Column::Id.is_in(req.word_ids))
            .all(state.db.as_ref())
            .await?;

        let affected = words.len();

        for word in &words {
            for tag_id in &req.remove_tag_ids {
                entity::word_tags::Entity::delete_many()
                    .filter(entity::word_tags::Column::WordId.eq(word.id))
                    .filter(entity::word_tags::Column::TagId.eq(*tag_id))
                    .exec(state.db.as_ref())
                    .await?;
            }

            for tag_id in &req.add_tag_ids {
                let exists = entity::word_tags::Entity::find()
                    .filter(entity::word_tags::Column::WordId.eq(word.id))
                    .filter(entity::word_tags::Column::TagId.eq(*tag_id))
                    .one(state.db.as_ref())
                    .await?;

                if exists.is_none() {
                    let word_tag = entity::word_tags::ActiveModel {
                        word_id: Set(word.id),
                        tag_id: Set(*tag_id),
                    };
                    word_tag.insert(state.db.as_ref()).await?;
                }
            }
        }

        Ok(Json(BatchOperationResponse { affected }))
    }
}
