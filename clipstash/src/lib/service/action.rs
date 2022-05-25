use crate::data::{query, DatabasePool, Transaction};
use crate::service::ask;
use crate::{Clip, ServiceError, ShortCode};
use std::convert::TryInto;

pub async fn new_clip(req: ask::NewClip, pool: &DatabasePool) 
    -> Result<Clip, ServiceError> {
        // try into will attempt to convert database model into clip
        Ok(query::new_clip(req, pool).await?.try_into()?)
    }

pub async fn update_clip(req: ask::UpdateClip, pool: &DatabasePool) 
    -> Result<Clip, ServiceError> {
        // try into will attempt to convert database model into clip
        Ok(query::update_clip(req, pool).await?.try_into()?)
    }
pub async fn get_clip(req: ask::GetClip, pool: &DatabasePool) 
    -> Result<Clip, ServiceError> {
        let user_password = req.password.clone();
        let clip: Clip = query::get_clip(req, pool).await?.try_into()?;
        if clip.password.has_password() {
            if clip.password == user_password {
                Ok(clip)
            } else {
                Err(ServiceError::PermissionError("invalid password".to_string()))
            }
        } else {
            Ok(clip)
        }
    }