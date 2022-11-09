use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPostReq {
    pub title: String,
    pub body: String,
    pub created_at: Option<SystemTime>,
    pub created_id: Option<i64>,
    pub updated_at: Option<SystemTime>,
    pub updated_id: Option<i64>,
}
