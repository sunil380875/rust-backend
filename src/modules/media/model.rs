use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// MongoDB document for the `media` collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    /// UUID-based filename on disk (e.g. `a1b2c3.jpg`).
    pub filename: String,

    /// Original filename from the client.
    pub original_name: String,

    /// MIME type.
    pub mime_type: String,

    /// File size in bytes.
    pub size: u64,

    /// ObjectId of the owning user.
    pub owner_id: ObjectId,

    pub created_at: DateTime<Utc>,
}
