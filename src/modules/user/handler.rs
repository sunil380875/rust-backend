use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use validator::Validate;

use crate::{
    error::{AppError, AppResult},
    modules::{
        auth::dto::Claims,
        user::{dto::{UpdateUserDto, UserResponse}, service::UserService},
    },
    state::AppState,
};

/// GET /api/users  (admin only)
pub async fn list_users(State(state): State<AppState>) -> AppResult<Json<Vec<UserResponse>>> {
    Ok(Json(UserService::list_all(&state).await?))
}

/// GET /api/users/:id
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<UserResponse>> {
    Ok(Json(UserService::get(&state, &id).await?))
}

/// PATCH /api/users/:id  (own profile)
pub async fn update_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(dto): Json<UpdateUserDto>,
) -> AppResult<Json<UserResponse>> {
    // Users can only update their own profile; admins can update any
    if claims.sub != id {
        use crate::modules::user::model::Role;
        if claims.role != Role::Admin {
            return Err(AppError::Forbidden(
                "You can only update your own profile".into(),
            ));
        }
    }
    dto.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    Ok(Json(UserService::update(&state, &id, dto).await?))
}

/// DELETE /api/users/:id  (admin only)
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    UserService::delete(&state, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}
