use axum::body::Body;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::header;
use axum::http::StatusCode;
use axum::response::Response;
use axum::Json;
use axum_extra::extract::Multipart;
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

use crate::auth::session::UserSession;
use crate::error::AppError;
use crate::import::data::ImportChapter;
use crate::import::data::ImportWordbook;
use crate::import::error::ImportError;
use crate::import::parser_excel::ExcelParser;
use crate::import::parser_json::JsonParser;
use crate::import::parser_xml::XmlParser;
use crate::import::template::TemplateGenerator;
use crate::state::AppState;

const FORMAT_JSON: &str = "json";
const FORMAT_XML: &str = "xml";
const FORMAT_XLSX: &str = "xlsx";
const FORMAT_CSV: &str = "csv";
const TARGET_WORDBOOK: &str = "wordbook";
const TARGET_CHAPTER: &str = "chapter";
const CONTENT_TYPE_JSON: &str = "application/json";
const CONTENT_TYPE_XML: &str = "application/xml";
const CONTENT_TYPE_XLSX: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
const CONTENT_TYPE_CSV: &str = "text/csv; charset=utf-8";

#[derive(Debug, Deserialize)]
pub struct TemplateQuery {
    pub format: String,
    pub target: String,
}

#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub chapters_created: usize,
    pub words_created: usize,
}

pub struct ImportHandler;

impl ImportHandler {
    async fn get_user_id(session: &Session) -> Result<i32, AppError> {
        UserSession::get(session)
            .await?
            .ok_or(AppError::Unauthorized)
    }

    pub async fn download_template(Query(query): Query<TemplateQuery>) -> Result<Response, AppError> {
        let (content, content_type, filename) = match (query.format.as_str(), query.target.as_str())
        {
            (FORMAT_JSON, TARGET_WORDBOOK) => (
                TemplateGenerator::json_wordbook_template().into_bytes(),
                CONTENT_TYPE_JSON,
                "wordbook_template.json",
            ),
            (FORMAT_JSON, TARGET_CHAPTER) => (
                TemplateGenerator::json_chapter_template().into_bytes(),
                CONTENT_TYPE_JSON,
                "chapter_template.json",
            ),
            (FORMAT_XML, TARGET_WORDBOOK) => (
                TemplateGenerator::xml_wordbook_template().into_bytes(),
                CONTENT_TYPE_XML,
                "wordbook_template.xml",
            ),
            (FORMAT_XML, TARGET_CHAPTER) => (
                TemplateGenerator::xml_chapter_template().into_bytes(),
                CONTENT_TYPE_XML,
                "chapter_template.xml",
            ),
            (FORMAT_XLSX, TARGET_WORDBOOK) => (
                TemplateGenerator::xlsx_wordbook_template(),
                CONTENT_TYPE_XLSX,
                "wordbook_template.xlsx",
            ),
            (FORMAT_XLSX, TARGET_CHAPTER) => (
                TemplateGenerator::xlsx_chapter_template(),
                CONTENT_TYPE_XLSX,
                "chapter_template.xlsx",
            ),
            (FORMAT_CSV, TARGET_WORDBOOK) => (
                TemplateGenerator::csv_wordbook_template().into_bytes(),
                CONTENT_TYPE_CSV,
                "wordbook_template.csv",
            ),
            (FORMAT_CSV, TARGET_CHAPTER) => (
                TemplateGenerator::csv_chapter_template().into_bytes(),
                CONTENT_TYPE_CSV,
                "chapter_template.csv",
            ),
            _ => {
                return Err(AppError::Validation(
                    "Invalid format or target".to_string(),
                ))
            }
        };

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, content_type)
            .header(
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename),
            )
            .body(Body::from(content))
            .unwrap())
    }

    pub async fn import_wordbook(
        State(state): State<AppState>,
        session: Session,
        mut multipart: Multipart,
    ) -> Result<Json<ImportResult>, AppError> {
        let user_id = Self::get_user_id(&session).await?;

        let mut file_data: Option<Vec<u8>> = None;
        let mut file_name: Option<String> = None;
        let mut wordbook_name: Option<String> = None;

        while let Some(field) = multipart.next_field().await.map_err(|e| {
            AppError::Validation(format!("Failed to read multipart field: {}", e))
        })? {
            let name = field.name().unwrap_or_default().to_string();
            match name.as_str() {
                "file" => {
                    file_name = field.file_name().map(String::from);
                    file_data = Some(field.bytes().await.map_err(|e| {
                        AppError::Validation(format!("Failed to read file: {}", e))
                    })?.to_vec());
                }
                "name" => {
                    wordbook_name = Some(
                        field
                            .text()
                            .await
                            .map_err(|e| AppError::Validation(format!("Failed to read name: {}", e)))?,
                    );
                }
                _ => {}
            }
        }

        let data = file_data.ok_or_else(|| AppError::Validation("No file uploaded".to_string()))?;
        let name = file_name.unwrap_or_else(|| "unknown".to_string());
        let wb_name = wordbook_name.unwrap_or_else(|| "Imported Wordbook".to_string());

        let wordbook = Self::parse_wordbook(&data, &name, wb_name)?;
        Self::save_wordbook(&state, user_id, wordbook).await
    }

    pub async fn import_chapter(
        State(state): State<AppState>,
        session: Session,
        Path(wordbook_id): Path<i32>,
        mut multipart: Multipart,
    ) -> Result<Json<ImportResult>, AppError> {
        let user_id = Self::get_user_id(&session).await?;

        entity::wordbooks::Entity::find_by_id(wordbook_id)
            .filter(entity::wordbooks::Column::UserId.eq(user_id))
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Wordbook not found".to_string()))?;

        let mut file_data: Option<Vec<u8>> = None;
        let mut file_name: Option<String> = None;
        let mut chapter_name: Option<String> = None;

        while let Some(field) = multipart.next_field().await.map_err(|e| {
            AppError::Validation(format!("Failed to read multipart field: {}", e))
        })? {
            let name = field.name().unwrap_or_default().to_string();
            match name.as_str() {
                "file" => {
                    file_name = field.file_name().map(String::from);
                    file_data = Some(field.bytes().await.map_err(|e| {
                        AppError::Validation(format!("Failed to read file: {}", e))
                    })?.to_vec());
                }
                "name" => {
                    chapter_name = Some(
                        field
                            .text()
                            .await
                            .map_err(|e| AppError::Validation(format!("Failed to read name: {}", e)))?,
                    );
                }
                _ => {}
            }
        }

        let data = file_data.ok_or_else(|| AppError::Validation("No file uploaded".to_string()))?;
        let name = file_name.unwrap_or_else(|| "unknown".to_string());
        let ch_name = chapter_name.unwrap_or_else(|| "Imported Chapter".to_string());

        let chapter = Self::parse_chapter(&data, &name, ch_name)?;
        Self::save_chapter(&state, wordbook_id, chapter).await
    }

    fn parse_wordbook(
        data: &[u8],
        filename: &str,
        wordbook_name: String,
    ) -> Result<ImportWordbook, AppError> {
        let ext = filename
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase();

        let result = match ext.as_str() {
            FORMAT_JSON => JsonParser::parse_wordbook(data),
            FORMAT_XML => XmlParser::parse_wordbook(data),
            "xlsx" | "xls" => ExcelParser::parse_wordbook(data, wordbook_name),
            "tsv" | "csv" => Self::parse_tsv_wordbook(data, wordbook_name),
            _ => Err(ImportError::InvalidFormat(format!(
                "Unsupported file format: {}",
                ext
            ))),
        };

        result.map_err(|e| AppError::Validation(e.to_string()))
    }

    fn parse_chapter(
        data: &[u8],
        filename: &str,
        chapter_name: String,
    ) -> Result<ImportChapter, AppError> {
        let ext = filename
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase();

        let result = match ext.as_str() {
            FORMAT_JSON => JsonParser::parse_chapter(data),
            FORMAT_XML => XmlParser::parse_chapter(data),
            "xlsx" | "xls" => ExcelParser::parse_chapter(data, chapter_name),
            "tsv" | "csv" => Self::parse_tsv_chapter(data, chapter_name),
            _ => Err(ImportError::InvalidFormat(format!(
                "Unsupported file format: {}",
                ext
            ))),
        };

        result.map_err(|e| AppError::Validation(e.to_string()))
    }

    fn parse_tsv_wordbook(data: &[u8], wordbook_name: String) -> Result<ImportWordbook, ImportError> {
        let content = std::str::from_utf8(data)
            .map_err(|e| ImportError::ParseError(e.to_string()))?;

        let mut lines = content.lines();
        let _header = lines.next().ok_or_else(|| {
            ImportError::InvalidFormat("Empty file".to_string())
        })?;

        let mut chapters_map: std::collections::HashMap<String, Vec<crate::import::data::ImportWord>> =
            std::collections::HashMap::new();

        for (idx, line) in lines.enumerate() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 3 {
                continue;
            }

            let chapter_name = parts[0].trim().to_string();
            let source = parts[1].trim().to_string();
            let translation = parts[2].trim().to_string();
            let note = parts.get(3).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());

            if source.is_empty() || translation.is_empty() {
                return Err(ImportError::invalid_data(idx + 2, "Missing source or translation"));
            }

            chapters_map
                .entry(chapter_name)
                .or_default()
                .push(crate::import::data::ImportWord::new(source, translation, note));
        }

        let chapters = chapters_map
            .into_iter()
            .map(|(name, words)| ImportChapter::with_words(name, words))
            .collect();

        Ok(ImportWordbook {
            name: wordbook_name,
            description: None,
            chapters,
        })
    }

    fn parse_tsv_chapter(data: &[u8], chapter_name: String) -> Result<ImportChapter, ImportError> {
        let content = std::str::from_utf8(data)
            .map_err(|e| ImportError::ParseError(e.to_string()))?;

        let mut lines = content.lines();
        let _header = lines.next().ok_or_else(|| {
            ImportError::InvalidFormat("Empty file".to_string())
        })?;

        let mut words = Vec::new();

        for (idx, line) in lines.enumerate() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 2 {
                continue;
            }

            let source = parts[0].trim().to_string();
            let translation = parts[1].trim().to_string();
            let note = parts.get(2).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());

            if source.is_empty() || translation.is_empty() {
                return Err(ImportError::invalid_data(idx + 2, "Missing source or translation"));
            }

            words.push(crate::import::data::ImportWord::new(source, translation, note));
        }

        Ok(ImportChapter::with_words(chapter_name, words))
    }

    async fn save_wordbook(
        state: &AppState,
        user_id: i32,
        wordbook: ImportWordbook,
    ) -> Result<Json<ImportResult>, AppError> {
        let now = Utc::now().fixed_offset();

        let max_order = entity::wordbooks::Entity::find()
            .filter(entity::wordbooks::Column::UserId.eq(user_id))
            .order_by_desc(entity::wordbooks::Column::SortOrder)
            .one(state.db.as_ref())
            .await?
            .map(|w| w.sort_order + 1)
            .unwrap_or(0);

        let wb = entity::wordbooks::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            name: Set(wordbook.name),
            description: Set(wordbook.description),
            cover_url: Set(None),
            sort_order: Set(max_order),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let saved_wb = wb.insert(state.db.as_ref()).await?;

        let mut chapters_created = 0;
        let mut words_created = 0;

        for (ch_idx, chapter) in wordbook.chapters.into_iter().enumerate() {
            let ch = entity::chapters::ActiveModel {
                id: NotSet,
                wordbook_id: Set(saved_wb.id),
                name: Set(chapter.name),
                sort_order: Set(ch_idx as i32),
                created_at: Set(now),
                updated_at: Set(now),
            };

            let saved_ch = ch.insert(state.db.as_ref()).await?;
            chapters_created += 1;

            for (w_idx, word) in chapter.words.into_iter().enumerate() {
                let w = entity::words::ActiveModel {
                    id: NotSet,
                    chapter_id: Set(saved_ch.id),
                    source: Set(word.source),
                    translation: Set(word.translation),
                    note: Set(word.note),
                    sort_order: Set(w_idx as i32),
                    created_at: Set(now),
                    updated_at: Set(now),
                };

                w.insert(state.db.as_ref()).await?;
                words_created += 1;
            }
        }

        Ok(Json(ImportResult {
            chapters_created,
            words_created,
        }))
    }

    async fn save_chapter(
        state: &AppState,
        wordbook_id: i32,
        chapter: ImportChapter,
    ) -> Result<Json<ImportResult>, AppError> {
        let now = Utc::now().fixed_offset();

        let max_order = entity::chapters::Entity::find()
            .filter(entity::chapters::Column::WordbookId.eq(wordbook_id))
            .order_by_desc(entity::chapters::Column::SortOrder)
            .one(state.db.as_ref())
            .await?
            .map(|c| c.sort_order + 1)
            .unwrap_or(0);

        let ch = entity::chapters::ActiveModel {
            id: NotSet,
            wordbook_id: Set(wordbook_id),
            name: Set(chapter.name),
            sort_order: Set(max_order),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let saved_ch = ch.insert(state.db.as_ref()).await?;

        let mut words_created = 0;

        for (w_idx, word) in chapter.words.into_iter().enumerate() {
            let w = entity::words::ActiveModel {
                id: NotSet,
                chapter_id: Set(saved_ch.id),
                source: Set(word.source),
                translation: Set(word.translation),
                note: Set(word.note),
                sort_order: Set(w_idx as i32),
                created_at: Set(now),
                updated_at: Set(now),
            };

            w.insert(state.db.as_ref()).await?;
            words_created += 1;
        }

        Ok(Json(ImportResult {
            chapters_created: 1,
            words_created,
        }))
    }
}