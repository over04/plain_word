use calamine::Data;
use calamine::Reader;
use calamine::Xlsx;
use std::collections::HashMap;
use std::io::Cursor;

use crate::import::data::ImportChapter;
use crate::import::data::ImportWord;
use crate::import::data::ImportWordbook;
use crate::import::error::ImportError;

const HEADER_CHAPTER_NAME: &str = "chapter_name";
const HEADER_SOURCE: &str = "source";
const HEADER_TRANSLATION: &str = "translation";
const HEADER_NOTE: &str = "note";

pub struct ExcelParser;

impl ExcelParser {
    pub fn parse_wordbook(data: &[u8], wordbook_name: String) -> Result<ImportWordbook, ImportError> {
        let cursor = Cursor::new(data);
        let mut workbook: Xlsx<_> = Xlsx::new(cursor)?;

        let sheet_name = workbook
            .sheet_names()
            .first()
            .cloned()
            .ok_or_else(|| ImportError::InvalidFormat("No sheets found".to_string()))?;

        let range = workbook
            .worksheet_range(&sheet_name)
            .map_err(|e| ImportError::ExcelError(e.to_string()))?;

        let mut rows = range.rows();
        let header = rows
            .next()
            .ok_or_else(|| ImportError::InvalidFormat("Empty sheet".to_string()))?;

        let col_indices = Self::parse_header(header, true)?;

        let mut chapters_map: HashMap<String, Vec<ImportWord>> = HashMap::new();

        for (row_idx, row) in rows.enumerate() {
            let chapter_name = Self::get_cell_string(row, col_indices.chapter_name.unwrap_or(0))
                .ok_or_else(|| ImportError::invalid_data(row_idx + 2, "Missing chapter_name"))?;

            let source = Self::get_cell_string(row, col_indices.source)
                .ok_or_else(|| ImportError::invalid_data(row_idx + 2, "Missing source"))?;

            let translation = Self::get_cell_string(row, col_indices.translation)
                .ok_or_else(|| ImportError::invalid_data(row_idx + 2, "Missing translation"))?;

            let note = col_indices.note.and_then(|idx| Self::get_cell_string(row, idx));

            let word = ImportWord::new(source, translation, note);
            chapters_map
                .entry(chapter_name)
                .or_default()
                .push(word);
        }

        let chapters: Vec<ImportChapter> = chapters_map
            .into_iter()
            .map(|(name, words)| ImportChapter::with_words(name, words))
            .collect();

        Ok(ImportWordbook {
            name: wordbook_name,
            description: None,
            chapters,
        })
    }

    pub fn parse_chapter(data: &[u8], chapter_name: String) -> Result<ImportChapter, ImportError> {
        let cursor = Cursor::new(data);
        let mut workbook: Xlsx<_> = Xlsx::new(cursor)?;

        let sheet_name = workbook
            .sheet_names()
            .first()
            .cloned()
            .ok_or_else(|| ImportError::InvalidFormat("No sheets found".to_string()))?;

        let range = workbook
            .worksheet_range(&sheet_name)
            .map_err(|e| ImportError::ExcelError(e.to_string()))?;

        let mut rows = range.rows();
        let header = rows
            .next()
            .ok_or_else(|| ImportError::InvalidFormat("Empty sheet".to_string()))?;

        let col_indices = Self::parse_header(header, false)?;

        let mut words = Vec::new();

        for (row_idx, row) in rows.enumerate() {
            let source = Self::get_cell_string(row, col_indices.source)
                .ok_or_else(|| ImportError::invalid_data(row_idx + 2, "Missing source"))?;

            let translation = Self::get_cell_string(row, col_indices.translation)
                .ok_or_else(|| ImportError::invalid_data(row_idx + 2, "Missing translation"))?;

            let note = col_indices.note.and_then(|idx| Self::get_cell_string(row, idx));

            words.push(ImportWord::new(source, translation, note));
        }

        Ok(ImportChapter::with_words(chapter_name, words))
    }

    fn parse_header(header: &[Data], require_chapter: bool) -> Result<ColumnIndices, ImportError> {
        let mut indices = ColumnIndices::default();

        for (idx, cell) in header.iter().enumerate() {
            if let Some(name) = Self::data_to_string(cell) {
                match name.to_lowercase().trim() {
                    HEADER_CHAPTER_NAME => indices.chapter_name = Some(idx),
                    HEADER_SOURCE => indices.source = idx,
                    HEADER_TRANSLATION => indices.translation = idx,
                    HEADER_NOTE => indices.note = Some(idx),
                    _ => {}
                }
            }
        }

        if require_chapter && indices.chapter_name.is_none() {
            return Err(ImportError::MissingField("chapter_name".to_string()));
        }

        Ok(indices)
    }

    fn get_cell_string(row: &[Data], idx: usize) -> Option<String> {
        row.get(idx).and_then(Self::data_to_string)
    }

    fn data_to_string(data: &Data) -> Option<String> {
        match data {
            Data::String(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            }
            Data::Float(f) => Some(f.to_string()),
            Data::Int(i) => Some(i.to_string()),
            Data::Bool(b) => Some(b.to_string()),
            Data::DateTime(dt) => Some(dt.to_string()),
            Data::DateTimeIso(s) => Some(s.clone()),
            Data::DurationIso(s) => Some(s.clone()),
            Data::Error(_) | Data::Empty => None,
        }
    }
}

#[derive(Default)]
struct ColumnIndices {
    chapter_name: Option<usize>,
    source: usize,
    translation: usize,
    note: Option<usize>,
}