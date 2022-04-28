// structure fields part of the clip
pub mod field;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]

pub enum ClipError {
    // performing custom error messages on the webpage
    #[error("invalid password: {0}")]
    InvalidPassword(String),
    #[error("invalid Title: {0}")]
    InvalidTitle(String),
    #[error("Empty Content")]
    EmptyContent,
    #[error("invalid Date: {0}")]
    InvalidDate(String),
    #[error("Date parse error")]
    DateParse(#[from] chrono::ParseError),
    #[error("invalid Id: {0}")]
    Id(#[from] uuid::Error),
    #[error("invalid hits: {0}")]
    Hits(#[from] std::num::TryFromIntError)
}
// All around clip storage here
// all accessed in filed module
// serialize is part of serde crate and need to include
// All these fields represent functions of the webapp 
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clip {
    pub clip_id: field::ClipId,
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
}

