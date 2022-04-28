// structure fields part of the clip
pub mod field;

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