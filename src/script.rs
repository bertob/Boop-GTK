use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Script {
    metadata: Metadata,
    source: String,
}

#[derive(Debug)]
pub enum ParseScriptError {
    NoMetadata,
    InvalidMetadata(serde_json::error::Error),
}

impl fmt::Display for ParseScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseScriptError::NoMetadata => write!(f, "no metadata"),
            ParseScriptError::InvalidMetadata(e) => write!(f, "invalid metadata: {}", e),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub api: u32,
    pub name: String,
    pub description: String,
    pub author: String,
    pub icon: String,
    pub tags: String,
}

impl Script {
    pub fn from_source(source: String) -> Result<Self, ParseScriptError> {
        let start = source.find("/**").ok_or(ParseScriptError::NoMetadata)?;
        let end = source.find("**/").ok_or(ParseScriptError::NoMetadata)?;

        let mut metadata: Metadata = serde_json::from_str(&source[start + 3..end])
            .map_err(ParseScriptError::InvalidMetadata)?;

        metadata.icon = metadata.icon.to_lowercase();

        Ok(Script { metadata, source })
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}
