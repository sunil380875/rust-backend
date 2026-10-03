use bson::{doc, oid::ObjectId};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::Collection;

use crate::{
    error::{AppError, AppResult},
    modules::user::{
        dto::{UpdateUserDto, UserResponse},
        model::User,
    },
    state::AppState,
};

pub struct UserService;

impl UserService {
    fn col(state: &AppState) -> Collection<User> {
        state.db.collection::<User>("users")
    }

    fn to_response(u: User) -> UserResponse {
        UserResponse {
            id: u.id.map(|id| id.to_hex()).unwrap_or_default(),
            name: u.name,
            email: u.email,
            role: u.role,
            avatar_url: u.avatar_url,
            created_at: u.created_at.to_rfc3339(),
            updated_at: u.updated_at.to_rfc3339(),
        }
    }

    /// Find a user by hex ObjectId string.
    pub async fn find_by_id(state: &AppState, id: &str) -> AppResult<User> {
        let oid = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest(format!("Invalid id: {id}")))?;

        Self::col(state)
            .find_one(doc! { "_id": oid })
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User {id} not found")))
    }

    /// List all users (admin).
    pub async fn list_all(state: &AppState) -> AppResult<Vec<UserResponse>> {
        let cursor = Self::col(state).find(doc! {}).await?;
        let users: Vec<User> = cursor.try_collect().await?;
        Ok(users.into_iter().map(Self::to_response).collect())
    }

    /// Get a single user by id.
    pub async fn get(state: &AppState, id: &str) -> AppResult<UserResponse> {
        Ok(Self::to_response(Self::find_by_id(state, id).await?))
    }

    /// Update the authenticated user's profile.
    pub async fn update(
        state: &AppState,
        id: &str,
        dto: UpdateUserDto,
    ) -> AppResult<UserResponse> {
        let oid = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest(format!("Invalid id: {id}")))?;

        let mut set_doc = doc! { "updated_at": Utc::now() };
        if let Some(name) = &dto.name {
            set_doc.insert("name", name);
        }
        if let Some(url) = &dto.avatar_url {
            set_doc.insert("avatar_url", url);
        }

        Self::col(state)
            .update_one(doc! { "_id": oid }, doc! { "$set": set_doc })
            .await?;

        Self::get(state, id).await
    }

    /// Delete a user by id (admin only).
    pub async fn delete(state: &AppState, id: &str) -> AppResult<()> {
        let oid = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest(format!("Invalid id: {id}")))?;

        let result = Self::col(state)
            .delete_one(doc! { "_id": oid })
            .await?;

        if result.deleted_count == 0 {
            return Err(AppError::NotFound(format!("User {id} not found")));
        }
        Ok(())
    }
}
