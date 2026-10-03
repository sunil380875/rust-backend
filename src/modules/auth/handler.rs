use axum::{extract::State, Extension, Json};
use validator::Validate;

use crate::{
    error::{AppError, AppResult},
    modules::auth::{
        dto::{AuthResponse, Claims, LoginDto, RegisterDto, UserPayload},
        service::AuthService,
    },
    modules::user::service::UserService,
    state::AppState,
};

/// POST /api/auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(dto): Json<RegisterDto>,
) -> AppResult<Json<AuthResponse>> {
    dto.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let resp = AuthService::register(&state, dto).await?;
    Ok(Json(resp))
}

/// POST /api/auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(dto): Json<LoginDto>,
) -> AppResult<Json<AuthResponse>> {
    dto.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let resp = AuthService::login(&state, dto).await?;
    Ok(Json(resp))
}

/// GET /api/auth/me  (protected)
pub async fn me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> AppResult<Json<UserPayload>> {
    let user = UserService::find_by_id(&state, &claims.sub).await?;
    Ok(Json(UserPayload {
        id: claims.sub,
        name: user.name,
        email: user.email,
        role: user.role,
        avatar_url: user.avatar_url,
    }))
}
