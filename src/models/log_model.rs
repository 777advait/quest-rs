use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::vault_model::Vault;

#[derive(Debug)]
pub struct Log {
    pub id: String,
    pub title: String,
    pub content: String,
    pub vault: Vault,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateLog {
    pub title: String,
    pub content: String,
    pub vault: Vault,
}

impl Log {
    pub fn new(data: CreateLog) -> Self {
        let vault_name = &data.vault.name;
        Self {
            id: format!(
                "{}/{}",
                vault_name,
                data.title.to_lowercase().replace(" ", "_")
            ),
            content: data.content,
            title: data.title,
            vault: data.vault,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl CreateLog {
    pub fn new(title: String, content: String, vault: Vault) -> Self {
        Self {
            title,
            content,
            vault,
        }
    }
}
