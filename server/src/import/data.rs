use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportWord {
    pub source: String,
    pub translation: String,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportChapter {
    pub name: String,
    #[serde(default)]
    pub words: Vec<ImportWord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportWordbook {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub chapters: Vec<ImportChapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "wordbook")]
pub struct XmlWordbook {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default, rename = "chapter")]
    pub chapters: Vec<XmlChapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlChapter {
    pub name: String,
    #[serde(default, rename = "word")]
    pub words: Vec<XmlWord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlWord {
    pub source: String,
    pub translation: String,
    #[serde(default)]
    pub note: Option<String>,
}

impl ImportWord {
    pub fn new(source: String, translation: String, note: Option<String>) -> Self {
        Self {
            source,
            translation,
            note,
        }
    }
}

impl ImportChapter {
    pub fn with_words(name: String, words: Vec<ImportWord>) -> Self {
        Self { name, words }
    }
}

impl From<XmlWordbook> for ImportWordbook {
    fn from(xml: XmlWordbook) -> Self {
        Self {
            name: xml.name,
            description: xml.description,
            chapters: xml.chapters.into_iter().map(ImportChapter::from).collect(),
        }
    }
}

impl From<XmlChapter> for ImportChapter {
    fn from(xml: XmlChapter) -> Self {
        Self {
            name: xml.name,
            words: xml.words.into_iter().map(ImportWord::from).collect(),
        }
    }
}

impl From<XmlWord> for ImportWord {
    fn from(xml: XmlWord) -> Self {
        Self {
            source: xml.source,
            translation: xml.translation,
            note: xml.note,
        }
    }
}