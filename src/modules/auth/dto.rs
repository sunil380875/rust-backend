use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::modules::user::model::Role;

// ── JWT Claims ────────────────────────────────────────────────────────────────

/// Claims embedded in the JWT token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject: the user's MongoDB ObjectId as a string.
    pub sub: String,
    /// User role.
    pub role: Role,
    /// Expiry timestamp (Unix seconds).
    pub exp: usize,
}

// ── Register ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterDto {
    #[validate(length(min = 2, max = 100))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

// ── Login ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub password: String,
}

// ── Responses ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserPayload,
}

/// Public user payload returned in auth responses (no password_hash).
#[derive(Debug, Serialize)]
pub struct UserPayload {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub avatar_url: Option<String>,
}
