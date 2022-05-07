use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};

// serialize and deserialize allow conversion to JSON for the API to ingest 
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if !content.trim().is_empty() {
            Ok(Self(content.to_string()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }
    // into inner is common on rust, accessing Content back into a string
    // This is common for converting for suitable into database
    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}