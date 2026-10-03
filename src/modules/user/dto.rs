use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::modules::user::model::Role;

/// Public user representation — never includes `password_hash`.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub avatar_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// DTO for updating a user's own profile.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,

    pub avatar_url: Option<String>,
}
