use async_trait::async_trait;
use color_eyre::Result;

use crate::{
    app::state::{SecretCollection, SecretItem},
    data::source::SecretSource,
};

#[derive(Debug, Default)]
pub struct MockSecretSource;

#[async_trait]
impl SecretSource for MockSecretSource {
    async fn load_collections(&self) -> Result<Vec<SecretCollection>> {
        Ok(vec![
            SecretCollection {
                id: String::from("development"),
                name: String::from("Development"),
                secret_key: String::from("mock://development"),
            },
            SecretCollection {
                id: String::from("infrastructure"),
                name: String::from("Infrastructure"),
                secret_key: String::from("mock://infrastructure"),
            },
        ])
    }

    async fn load_items(&self, collection: &SecretCollection) -> Result<Vec<SecretItem>> {
        let items = match collection.secret_key.as_str() {
            "mock://development" => vec![SecretItem {
                collection_id: collection.id.clone(),
                name: String::from("GitHub Token"),
                kind: String::from("API Token"),
                source: String::from("mock/local"),
                updated_at: String::from("2026-03-06"),
                tags: vec![String::from("dev"), String::from("git")],
            }],
            "mock://infrastructure" => vec![
                SecretItem {
                    collection_id: collection.id.clone(),
                    name: String::from("AWS Dev"),
                    kind: String::from("Cloud Credential"),
                    source: String::from("mock/local"),
                    updated_at: String::from("2026-03-01"),
                    tags: vec![String::from("infra"), String::from("aws")],
                },
                SecretItem {
                    collection_id: collection.id.clone(),
                    name: String::from("Database Staging"),
                    kind: String::from("Database Secret"),
                    source: String::from("mock/local"),
                    updated_at: String::from("2026-02-27"),
                    tags: vec![String::from("db"), String::from("staging")],
                },
            ],
            _ => Vec::new(),
        };

        Ok(items)
    }
}
