// /src/models.rs

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub channel: String,
    pub sender_id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
