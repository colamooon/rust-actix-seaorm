use sea_orm::DatabaseConnection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PageInfo {
    pub page: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}
