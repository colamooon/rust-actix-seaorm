use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PageInfo {
    pub page: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pageable<T> {
    pub content: Vec<T>,
    pub is_last: bool,
    pub page: u64,
    pub size: u64,
    pub total_count: u64,
    pub num_pages: u64,
}
