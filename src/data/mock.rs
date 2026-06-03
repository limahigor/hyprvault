use async_trait::async_trait;
use color_eyre::Result;

use crate::{app::state::SecretItem, data::source::SecretSource};

#[derive(Debug, Default)]
pub struct MockSecretSource;

#[async_trait]
impl SecretSource for MockSecretSource {
    async fn load_items(&self) -> Result<Vec<SecretItem>> {
        Ok(vec![
            SecretItem {
                name: String::from("GitHub Token"),
                kind: String::from("API Token"),
                source: String::from("mock/local"),
                updated_at: String::from("2026-03-06"),
                tags: vec![String::from("dev"), String::from("git")],
            },
            SecretItem {
                name: String::from("AWS Dev"),
                kind: String::from("Cloud Credential"),
                source: String::from("mock/local"),
                updated_at: String::from("2026-03-01"),
                tags: vec![String::from("infra"), String::from("aws")],
            },
            SecretItem {
                name: String::from("Database Staging"),
                kind: String::from("Database Secret"),
                source: String::from("mock/local"),
                updated_at: String::from("2026-02-27"),
                tags: vec![String::from("db"), String::from("staging")],
            },
        ])
    }
}
