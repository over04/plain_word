use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("Invalid file format: {0}")]
    InvalidFormat(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid data at row {row}: {message}")]
    InvalidData { row: usize, message: String },

    #[error("Excel error: {0}")]
    ExcelError(String),

    #[error("XML error: {0}")]
    XmlError(String),

    #[error("IO error: {0}")]
    IoError(String),
}

impl ImportError {
    pub fn invalid_data(row: usize, message: impl Into<String>) -> Self {
        Self::InvalidData {
            row,
            message: message.into(),
        }
    }
}

impl From<calamine::Error> for ImportError {
    fn from(e: calamine::Error) -> Self {
        Self::ExcelError(e.to_string())
    }
}

impl From<calamine::XlsxError> for ImportError {
    fn from(e: calamine::XlsxError) -> Self {
        Self::ExcelError(e.to_string())
    }
}

impl From<quick_xml::DeError> for ImportError {
    fn from(e: quick_xml::DeError) -> Self {
        Self::XmlError(e.to_string())
    }
}

impl From<serde_json::Error> for ImportError {
    fn from(e: serde_json::Error) -> Self {
        Self::ParseError(e.to_string())
    }
}

impl From<std::io::Error> for ImportError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e.to_string())
    }
}