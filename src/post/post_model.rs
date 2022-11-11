use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPostReq {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRes {
    pub id: i64,
    pub active: i8,
    pub created_at: Option<DateTimeUtc>,
    pub created_id: i64,
    pub updated_at: Option<DateTimeUtc>,
    pub updated_id: Option<i64>,
    pub title: String,
    pub content: String,
}
