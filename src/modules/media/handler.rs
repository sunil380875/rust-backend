use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    error::AppResult,
    middleware::file_interceptor::intercept_upload,
    modules::{auth::dto::Claims, media::service::{MediaResponse, MediaService}},
    state::AppState,
};

/// POST /api/media/upload  (auth required, multipart/form-data)
pub async fn upload_file(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    multipart: Multipart,
) -> AppResult<Json<MediaResponse>> {
    let uploaded = intercept_upload(&state, multipart).await?;
    let resp = MediaService::create(&state, uploaded, &claims.sub).await?;
    Ok(Json(resp))
}

/// GET /api/media  (auth required — lists caller's own files)
pub async fn list_files(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> AppResult<Json<Vec<MediaResponse>>> {
    Ok(Json(
        MediaService::list_by_owner(&state, &claims.sub).await?,
    ))
}

/// DELETE /api/media/:id  (auth required — owner or admin)
pub async fn delete_file(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    MediaService::delete(&state, &id, &claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}
