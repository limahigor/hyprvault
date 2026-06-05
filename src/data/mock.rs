use async_trait::async_trait;
use color_eyre::Result;

use crate::{
    app::state::{SecretAttribute, SecretCollection, SecretItem},
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
                updated_at: String::from("2026-03-06 09:30"),
                attributes: vec![
                    SecretAttribute {
                        key: String::from("account"),
                        value: String::from("higor"),
                    },
                    SecretAttribute {
                        key: String::from("service"),
                        value: String::from("github.com"),
                    },
                ],
            }],
            "mock://infrastructure" => vec![
                SecretItem {
                    collection_id: collection.id.clone(),
                    name: String::from("AWS Dev"),
                    kind: String::from("Cloud Credential"),
                    source: String::from("mock/local"),
                    updated_at: String::from("2026-03-01 18:45"),
                    attributes: vec![
                        SecretAttribute {
                            key: String::from("account"),
                            value: String::from("dev-account"),
                        },
                        SecretAttribute {
                            key: String::from("region"),
                            value: String::from("us-east-1"),
                        },
                    ],
                },
                SecretItem {
                    collection_id: collection.id.clone(),
                    name: String::from("Database Staging"),
                    kind: String::from("Database Secret"),
                    source: String::from("mock/local"),
                    updated_at: String::from("2026-02-27 14:10"),
                    attributes: vec![
                        SecretAttribute {
                            key: String::from("host"),
                            value: String::from("staging-db.internal"),
                        },
                        SecretAttribute {
                            key: String::from("username"),
                            value: String::from("staging_user"),
                        },
                    ],
                },
            ],
            _ => Vec::new(),
        };

        Ok(items)
    }
}
