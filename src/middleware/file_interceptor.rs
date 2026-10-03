//! File upload interceptor.
//!
//! Because Axum's `Multipart` consumes the request body, this cannot live as
//! a true `middleware::from_fn` layer (the body would be consumed before the
//! handler sees it).  Instead the logic is exposed as `intercept_upload`, a
//! helper called directly by the media handler, keeping validation/storage
//! logic centralised here.

use std::path::PathBuf;

use axum::extract::Multipart;
use uuid::Uuid;

use crate::{error::AppError, state::AppState};

/// Metadata produced after a file has been validated and written to disk.
#[derive(Debug, Clone)]
pub struct UploadedFile {
    pub filename: String,
    pub original_name: String,
    pub mime_type: String,
    pub size: u64,
    #[allow(dead_code)] // retained for future use (e.g. post-process hooks)
    pub path: PathBuf,
}

/// Allowed MIME types.
const ALLOWED_MIME: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/gif",
    "image/webp",
    "application/pdf",
    "video/mp4",
];

/// Validate, read, and persist the first file field in `multipart`.
/// Returns [`UploadedFile`] metadata on success.
pub async fn intercept_upload(
    state: &AppState,
    mut multipart: Multipart,
) -> Result<UploadedFile, AppError> {
    let max_bytes = state.config.max_upload_mb * 1024 * 1024;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Multipart error: {e}")))?
    {
        let original_name = field.file_name().unwrap_or("upload").to_string();
        let mime_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        // --- MIME allowlist ---
        if !ALLOWED_MIME.contains(&mime_type.as_str()) {
            return Err(AppError::BadRequest(format!(
                "File type '{}' is not allowed. Allowed: {}",
                mime_type,
                ALLOWED_MIME.join(", ")
            )));
        }

        // --- Read with size limit ---
        let data = field
            .bytes()
            .await
            .map_err(|e| AppError::BadRequest(format!("Failed to read file data: {e}")))?;

        if data.len() as u64 > max_bytes {
            return Err(AppError::BadRequest(format!(
                "File size {}B exceeds limit of {} MB",
                data.len(),
                state.config.max_upload_mb
            )));
        }

        // --- Determine extension from MIME ---
        let ext = mime_to_ext(&mime_type);
        let filename = format!("{}.{}", Uuid::new_v4(), ext);
        let path = PathBuf::from(&state.config.upload_dir).join(&filename);

        // --- Stream to disk ---
        tokio::fs::write(&path, &data)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to write file to disk: {e}")))?;

        return Ok(UploadedFile {
            filename,
            original_name,
            mime_type,
            size: data.len() as u64,
            path,
        });
    }

    Err(AppError::BadRequest(
        "No file field found in multipart body".into(),
    ))
}

fn mime_to_ext(mime: &str) -> &'static str {
    match mime {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "application/pdf" => "pdf",
        "video/mp4" => "mp4",
        _ => "bin",
    }
}
