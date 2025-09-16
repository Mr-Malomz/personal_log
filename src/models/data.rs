use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub id: i32,
    pub content: String,
    pub created_at: String,
}
