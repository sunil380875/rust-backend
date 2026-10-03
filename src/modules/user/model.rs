use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Role enum stored in MongoDB and embedded in JWT claims.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
}

/// MongoDB document for the `users` collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub name: String,
    pub email: String,

    /// Argon2 password hash — never exposed in responses.
    pub password_hash: String,

    #[serde(default = "default_role")]
    pub role: Role,

    pub avatar_url: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn default_role() -> Role {
    Role::User
}
