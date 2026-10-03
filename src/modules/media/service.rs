use bson::{doc, oid::ObjectId};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::{
    error::{AppError, AppResult},
    middleware::file_interceptor::UploadedFile,
    modules::media::model::Media,
    state::AppState,
};

/// Public response DTO for a media document.
#[derive(Debug, Serialize, Deserialize)]
pub struct MediaResponse {
    pub id: String,
    pub filename: String,
    pub original_name: String,
    pub mime_type: String,
    pub size: u64,
    pub owner_id: String,
    pub url: String,
    pub created_at: String,
}

pub struct MediaService;

impl MediaService {
    fn col(state: &AppState) -> Collection<Media> {
        state.db.collection::<Media>("media")
    }

    fn to_response(m: Media) -> MediaResponse {
        let id_str = m.id.map(|id| id.to_hex()).unwrap_or_default();
        let url = format!("/uploads/{}", m.filename);
        MediaResponse {
            id: id_str,
            filename: m.filename,
            original_name: m.original_name,
            mime_type: m.mime_type,
            size: m.size,
            owner_id: m.owner_id.to_hex(),
            url,
            created_at: m.created_at.to_rfc3339(),
        }
    }

    /// Persist file metadata after the interceptor has written the file to disk.
    pub async fn create(
        state: &AppState,
        uploaded: UploadedFile,
        owner_id: &str,
    ) -> AppResult<MediaResponse> {
        let oid = ObjectId::parse_str(owner_id)
            .map_err(|_| AppError::BadRequest("Invalid owner id".into()))?;

        let media = Media {
            id: None,
            filename: uploaded.filename,
            original_name: uploaded.original_name,
            mime_type: uploaded.mime_type,
            size: uploaded.size,
            owner_id: oid,
            created_at: Utc::now(),
        };

        let result = Self::col(state).insert_one(&media).await?;
        let inserted_id = result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| AppError::Internal("Failed to get inserted id".into()))?;

        Self::get(state, &inserted_id.to_hex()).await
    }

    /// List all media owned by a user.
    pub async fn list_by_owner(
        state: &AppState,
        owner_id: &str,
    ) -> AppResult<Vec<MediaResponse>> {
        let oid = ObjectId::parse_str(owner_id)
            .map_err(|_| AppError::BadRequest("Invalid owner id".into()))?;

        let cursor = Self::col(state)
            .find(doc! { "owner_id": oid })
            .await?;
        let items: Vec<Media> = cursor.try_collect().await?;
        Ok(items.into_iter().map(Self::to_response).collect())
    }

    /// Get a single media document.
    pub async fn get(state: &AppState, id: &str) -> AppResult<MediaResponse> {
        let oid = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest(format!("Invalid id: {id}")))?;

        let m = Self::col(state)
            .find_one(doc! { "_id": oid })
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Media {id} not found")))?;

        Ok(Self::to_response(m))
    }

    /// Delete a media document (and the file on disk).
    pub async fn delete(state: &AppState, id: &str, requester_id: &str) -> AppResult<()> {
        let oid = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest(format!("Invalid id: {id}")))?;

        let m = Self::col(state)
            .find_one(doc! { "_id": oid })
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Media {id} not found")))?;

        // Only owner or admin can delete
        if m.owner_id.to_hex() != requester_id {
            return Err(AppError::Forbidden(
                "You do not own this media file".into(),
            ));
        }

        // Delete from disk
        let path = std::path::PathBuf::from(&state.config.upload_dir).join(&m.filename);
        if path.exists() {
            tokio::fs::remove_file(&path).await?;
        }

        Self::col(state).delete_one(doc! { "_id": oid }).await?;
        Ok(())
    }
}
