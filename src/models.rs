use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Status {
    PENDING,
    DONE,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub done_at: Option<chrono::DateTime<chrono::Utc>>,
}
