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
        model::Clip,
        "SELECT * FROM clips WHERE shortcode = ?",
        shortcode
    )
    // will attempt to fetch one item from DB
    .fetch_one(pool)
    // will send this to db and work another item
    .await?)
}

pub async fn new_clip<M: Into<model::NewClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    // convert data from input
    let model = model.into();
    // r# is meant for raw string query
    let _ = sqlx::query!(
        model::Clip,
        r#"INSERT INTO clips (
            clip_id,
            shortcode,
            content,
            title,
            posted,
            expires,
            password,
            hits)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0
    )
    .execute(pool)
    .await?;
    // return data that was added to db, requires a structure
    // becuase passing a string it will automatically convert with these funcitons
    get_clip(model.shortcode, pool).await
}

pub async fn update_clip<M: Into<model::UpdateClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"UPDATE clips SET
            content = ?,
            expires = ?,
            password = ?,
            title = ?
            WHERE shortcode = ?"#,
        model.content,
        model.expires,
        model.password,
        model.title,
        model.shortcode
    )
    .execute(pool);
    .await?;
    get_clip(model.shortcode, pool).await
}