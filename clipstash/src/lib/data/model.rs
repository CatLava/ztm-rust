use crate::data::DbId;
use crate::{ClipError, ShortCode, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;

// clip used in model as well
// prior to adding to db need to implement transformations
#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub clip_id: String,
    // short code to access clip
    pub shortcode: String,
    pub content: String,
    pub title: Option<String>,
    // date posted
    pub posted: NaiveDateTime,
    pub expires: Option<NaiveDateTime>,
    pub password: Option<String>,
    // number of times seen on webpage
    pub hits: i64,
}

impl TryFrom<Clip> for crate::domain::Clip {
    type Error = ClipError;
    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        use std::str::FromStr;
        Ok(Self {
            clip_id: field::ClipId::new(DbId::from_str(clip.clip_id.as_str())?),
            // short code to access clip
            shortcode: field::ShortCode::from(clip.shortcode),
            content: field::Content::new(clip.content.as_str())?,
            title: field::Title::new(clip.title),
            // date posted
            posted: field::Posted::new(Time::from_naive_utc(clip.posted)),
            expires: field::Expires::new(clip.expires.map(Time::from_naive_utc)),
            password: field::Password::new(clip.password.unwrap_or_default())?,
            // number of times seen on webpage
            hits: field::Hits::new(u64::try_from(clip.hits)?),
        }

        )
    }
}

