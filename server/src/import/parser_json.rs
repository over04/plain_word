use crate::import::data::ImportChapter;
use crate::import::data::ImportWordbook;
use crate::import::error::ImportError;

pub struct JsonParser;

impl JsonParser {
    pub fn parse_wordbook(data: &[u8]) -> Result<ImportWordbook, ImportError> {
        let content = std::str::from_utf8(data)
            .map_err(|e| ImportError::ParseError(e.to_string()))?;
        serde_json::from_str(content).map_err(ImportError::from)
    }

    pub fn parse_chapter(data: &[u8]) -> Result<ImportChapter, ImportError> {
        let content = std::str::from_utf8(data)
            .map_err(|e| ImportError::ParseError(e.to_string()))?;
        serde_json::from_str(content).map_err(ImportError::from)
    }
}