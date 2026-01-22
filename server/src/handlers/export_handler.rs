use axum::body::Body;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::header;
use axum::http::StatusCode;
use axum::response::Response;
use rust_xlsxwriter::Workbook;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde::Serialize;
use tower_sessions::Session;

use crate::auth::session::UserSession;
use crate::error::AppError;
use crate::import::data::ImportChapter;
use crate::import::data::ImportWord;
use crate::import::data::ImportWordbook;
use crate::state::AppState;

const FORMAT_JSON: &str = "json";
const FORMAT_XML: &str = "xml";
const FORMAT_CSV: &str = "csv";
const FORMAT_XLSX: &str = "xlsx";
const CONTENT_TYPE_XLSX: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
const HEADER_CHAPTER_NAME: &str = "chapter_name";
const HEADER_SOURCE: &str = "source";
const HEADER_TRANSLATION: &str = "translation";
const HEADER_NOTE: &str = "note";

#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    pub format: String,
}

#[derive(Debug, Serialize)]
#[serde(rename = "wordbook")]
struct XmlExportWordbook {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "chapter")]
    chapters: Vec<XmlExportChapter>,
}

#[derive(Debug, Serialize)]
struct XmlExportChapter {
    name: String,
    #[serde(rename = "word")]
    words: Vec<XmlExportWord>,
}

#[derive(Debug, Serialize)]
struct XmlExportWord {
    source: String,
    translation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}

pub struct ExportHandler;

impl ExportHandler {
    async fn get_user_id(session: &Session) -> Result<i32, AppError> {
        UserSession::get(session)
            .await?
            .ok_or(AppError::Unauthorized)
    }

    pub async fn export_wordbook(
        State(state): State<AppState>,
        session: Session,
        Path(wordbook_id): Path<i32>,
        Query(query): Query<ExportQuery>,
    ) -> Result<Response, AppError> {
        let user_id = Self::get_user_id(&session).await?;

        let wordbook = entity::wordbooks::Entity::find_by_id(wordbook_id)
            .filter(entity::wordbooks::Column::UserId.eq(user_id))
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Wordbook not found".to_string()))?;

        let chapters = entity::chapters::Entity::find()
            .filter(entity::chapters::Column::WordbookId.eq(wordbook_id))
            .order_by_asc(entity::chapters::Column::SortOrder)
            .all(state.db.as_ref())
            .await?;

        let mut import_chapters = Vec::with_capacity(chapters.len());

        for chapter in chapters {
            let words = entity::words::Entity::find()
                .filter(entity::words::Column::ChapterId.eq(chapter.id))
                .order_by_asc(entity::words::Column::SortOrder)
                .all(state.db.as_ref())
                .await?;

            let import_words: Vec<ImportWord> = words
                .into_iter()
                .map(|w| ImportWord {
                    source: w.source,
                    translation: w.translation,
                    note: w.note,
                })
                .collect();

            import_chapters.push(ImportChapter::with_words(chapter.name, import_words));
        }

        let export_data = ImportWordbook {
            name: wordbook.name.clone(),
            description: wordbook.description,
            chapters: import_chapters,
        };

        Self::build_wordbook_response(&export_data, &wordbook.name, &query.format)
    }

    pub async fn export_chapter(
        State(state): State<AppState>,
        session: Session,
        Path((wordbook_id, chapter_id)): Path<(i32, i32)>,
        Query(query): Query<ExportQuery>,
    ) -> Result<Response, AppError> {
        let user_id = Self::get_user_id(&session).await?;

        entity::wordbooks::Entity::find_by_id(wordbook_id)
            .filter(entity::wordbooks::Column::UserId.eq(user_id))
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Wordbook not found".to_string()))?;

        let chapter = entity::chapters::Entity::find_by_id(chapter_id)
            .filter(entity::chapters::Column::WordbookId.eq(wordbook_id))
            .one(state.db.as_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Chapter not found".to_string()))?;

        let words = entity::words::Entity::find()
            .filter(entity::words::Column::ChapterId.eq(chapter_id))
            .order_by_asc(entity::words::Column::SortOrder)
            .all(state.db.as_ref())
            .await?;

        let import_words: Vec<ImportWord> = words
            .into_iter()
            .map(|w| ImportWord {
                source: w.source,
                translation: w.translation,
                note: w.note,
            })
            .collect();

        let export_data = ImportChapter::with_words(chapter.name.clone(), import_words);

        Self::build_chapter_response(&export_data, &chapter.name, &query.format)
    }

    fn build_wordbook_response(
        data: &ImportWordbook,
        name: &str,
        format: &str,
    ) -> Result<Response, AppError> {
        match format {
            FORMAT_JSON => {
                let content = serde_json::to_string_pretty(data)
                    .map_err(|e| AppError::Internal(e.to_string()))?;
                Self::build_response(
                    content.into_bytes(),
                    "application/json",
                    &format!("{}.json", name),
                )
            }
            FORMAT_XML => {
                let xml_data = XmlExportWordbook {
                    name: data.name.clone(),
                    description: data.description.clone(),
                    chapters: data
                        .chapters
                        .iter()
                        .map(|c| XmlExportChapter {
                            name: c.name.clone(),
                            words: c
                                .words
                                .iter()
                                .map(|w| XmlExportWord {
                                    source: w.source.clone(),
                                    translation: w.translation.clone(),
                                    note: w.note.clone(),
                                })
                                .collect(),
                        })
                        .collect(),
                };
                let content = Self::to_xml(&xml_data)?;
                Self::build_response(content.into_bytes(), "application/xml", &format!("{}.xml", name))
            }
            FORMAT_CSV => {
                let content = Self::wordbook_to_csv(data);
                Self::build_response(
                    content.into_bytes(),
                    "text/csv; charset=utf-8",
                    &format!("{}.csv", name),
                )
            }
            FORMAT_XLSX => {
                let content = Self::wordbook_to_xlsx(data);
                Self::build_response(content, CONTENT_TYPE_XLSX, &format!("{}.xlsx", name))
            }
            _ => Err(AppError::Validation(format!("Unsupported format: {}", format))),
        }
    }

    fn build_chapter_response(
        data: &ImportChapter,
        name: &str,
        format: &str,
    ) -> Result<Response, AppError> {
        match format {
            FORMAT_JSON => {
                let content = serde_json::to_string_pretty(data)
                    .map_err(|e| AppError::Internal(e.to_string()))?;
                Self::build_response(
                    content.into_bytes(),
                    "application/json",
                    &format!("{}.json", name),
                )
            }
            FORMAT_XML => {
                let xml_data = XmlExportChapter {
                    name: data.name.clone(),
                    words: data
                        .words
                        .iter()
                        .map(|w| XmlExportWord {
                            source: w.source.clone(),
                            translation: w.translation.clone(),
                            note: w.note.clone(),
                        })
                        .collect(),
                };
                let content = Self::to_xml(&xml_data)?;
                Self::build_response(content.into_bytes(), "application/xml", &format!("{}.xml", name))
            }
            FORMAT_CSV => {
                let content = Self::chapter_to_csv(data);
                Self::build_response(
                    content.into_bytes(),
                    "text/csv; charset=utf-8",
                    &format!("{}.csv", name),
                )
            }
            FORMAT_XLSX => {
                let content = Self::chapter_to_xlsx(data);
                Self::build_response(content, CONTENT_TYPE_XLSX, &format!("{}.xlsx", name))
            }
            _ => Err(AppError::Validation(format!("Unsupported format: {}", format))),
        }
    }

    fn build_response(content: Vec<u8>, content_type: &str, filename: &str) -> Result<Response, AppError> {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, content_type)
            .header(
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename),
            )
            .body(Body::from(content))
            .map_err(|e| AppError::Internal(e.to_string()))
    }

    fn to_xml<T: Serialize>(data: &T) -> Result<String, AppError> {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        let serialized = quick_xml::se::to_string(data)
            .map_err(|e| AppError::Internal(e.to_string()))?;
        xml.push_str(&serialized);
        Ok(xml)
    }

    fn wordbook_to_csv(data: &ImportWordbook) -> String {
        let mut content = String::from("chapter_name,source,translation,note\n");
        for chapter in &data.chapters {
            for word in &chapter.words {
                content.push_str(&Self::escape_csv(&chapter.name));
                content.push(',');
                content.push_str(&Self::escape_csv(&word.source));
                content.push(',');
                content.push_str(&Self::escape_csv(&word.translation));
                content.push(',');
                content.push_str(&Self::escape_csv(word.note.as_deref().unwrap_or("")));
                content.push('\n');
            }
        }
        content
    }

    fn chapter_to_csv(data: &ImportChapter) -> String {
        let mut content = String::from("source,translation,note\n");
        for word in &data.words {
            content.push_str(&Self::escape_csv(&word.source));
            content.push(',');
            content.push_str(&Self::escape_csv(&word.translation));
            content.push(',');
            content.push_str(&Self::escape_csv(word.note.as_deref().unwrap_or("")));
            content.push('\n');
        }
        content
    }

    fn escape_csv(value: &str) -> String {
        if value.contains(',') || value.contains('"') || value.contains('\n') {
            format!("\"{}\"", value.replace('"', "\"\""))
        } else {
            value.to_string()
        }
    }

    fn wordbook_to_xlsx(data: &ImportWordbook) -> Vec<u8> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        let _ = worksheet.write_string(0, 0, HEADER_CHAPTER_NAME);
        let _ = worksheet.write_string(0, 1, HEADER_SOURCE);
        let _ = worksheet.write_string(0, 2, HEADER_TRANSLATION);
        let _ = worksheet.write_string(0, 3, HEADER_NOTE);

        let mut row: u32 = 1;
        for chapter in &data.chapters {
            for word in &chapter.words {
                let _ = worksheet.write_string(row, 0, &chapter.name);
                let _ = worksheet.write_string(row, 1, &word.source);
                let _ = worksheet.write_string(row, 2, &word.translation);
                let _ = worksheet.write_string(row, 3, word.note.as_deref().unwrap_or(""));
                row += 1;
            }
        }

        workbook.save_to_buffer().unwrap_or_default()
    }

    fn chapter_to_xlsx(data: &ImportChapter) -> Vec<u8> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        let _ = worksheet.write_string(0, 0, HEADER_SOURCE);
        let _ = worksheet.write_string(0, 1, HEADER_TRANSLATION);
        let _ = worksheet.write_string(0, 2, HEADER_NOTE);

        for (idx, word) in data.words.iter().enumerate() {
            let row = (idx + 1) as u32;
            let _ = worksheet.write_string(row, 0, &word.source);
            let _ = worksheet.write_string(row, 1, &word.translation);
            let _ = worksheet.write_string(row, 2, word.note.as_deref().unwrap_or(""));
        }

        workbook.save_to_buffer().unwrap_or_default()
    }
}