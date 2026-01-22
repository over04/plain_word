use crate::import::data::ImportChapter;
use crate::import::data::ImportWordbook;
use crate::import::data::XmlChapter;
use crate::import::data::XmlWordbook;
use crate::import::error::ImportError;

pub struct XmlParser;

impl XmlParser {
    pub fn parse_wordbook(data: &[u8]) -> Result<ImportWordbook, ImportError> {
        let content = std::str::from_utf8(data)
            .map_err(|e| ImportError::ParseError(e.to_string()))?;
        let xml_wordbook: XmlWordbook = quick_xml::de::from_str(content)?;
        Ok(ImportWordbook::from(xml_wordbook))
    }

    pub fn parse_chapter(data: &[u8]) -> Result<ImportChapter, ImportError> {
        let content = std::str::from_utf8(data)
            .map_err(|e| ImportError::ParseError(e.to_string()))?;
        let xml_chapter: XmlChapter = quick_xml::de::from_str(content)?;
        Ok(ImportChapter::from(xml_chapter))
    }
}