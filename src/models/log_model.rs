use std::path::PathBuf;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Log {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub path: PathBuf,
}

pub struct CreateLog {
    pub title: String,
    pub content: String,
    pub path: PathBuf,
}

impl Log {
    pub fn new(data: CreateLog) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: data.content,
            title: data.title,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            path: data.path,
        }
    }
}
