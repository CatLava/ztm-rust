use super::model;
use crate::data::{DataError, DatabasePool};
use crate::ShortCode;
use sqlx::Row;

type Result<T> = std::result::Result<T, DataError>;

// function accepts generic type
// structure made in model,
pub async fn get_clip<M: Into<model::GetClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(sqlx::query_as!(
        model::Clip,,
        "SELECT * FROM clips WHERE shortcode = ?",
        shortcode
    )
    // will attempt to fetch one item from DB
    .fetch_one(pool)
    // will send this to db and work another item
    .await?)
}