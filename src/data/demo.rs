use async_trait::async_trait;
use color_eyre::{Result, eyre::eyre};

use crate::{
    app::state::{SecretAttribute, SecretCollection, SecretItem},
    data::source::SecretSource,
};

const HIDDEN_SECRET: &str = "************";

#[derive(Debug, Default)]
pub struct DemoSecretSource;

#[async_trait]
impl SecretSource for DemoSecretSource {
    async fn load_collections(&self) -> Result<Vec<SecretCollection>> {
        Ok(vec![
            SecretCollection {
                id: String::from("vault-personal"),
                name: String::from("Personal Vault"),
                secret_key: String::from("demo://vault-personal"),
            },
            SecretCollection {
                id: String::from("vault-work"),
                name: String::from("Work Vault"),
                secret_key: String::from("demo://vault-work"),
            },
            SecretCollection {
                id: String::from("vault-infra"),
                name: String::from("Infra Vault"),
                secret_key: String::from("demo://vault-infra"),
            },
        ])
    }

    async fn load_items(&self, collection: &SecretCollection) -> Result<Vec<SecretItem>> {
        Ok(match collection.id.as_str() {
            "vault-personal" => vec![
                demo_item(
                    collection.id.as_str(),
                    "item-github",
                    "GitHub",
                    "Password",
                    "2026-06-05 09:24",
                    vec![
                        attribute("service", "github.com"),
                        attribute("username", "limahigor"),
                        attribute("application", "browser"),
                    ],
                ),
                demo_item(
                    collection.id.as_str(),
                    "item-mail",
                    "Proton Mail",
                    "Password",
                    "2026-06-01 20:11",
                    vec![
                        attribute("service", "mail.proton.me"),
                        attribute("username", "higor@pm.me"),
                    ],
                ),
            ],
            "vault-work" => vec![
                demo_item(
                    collection.id.as_str(),
                    "item-figma",
                    "Figma Workspace",
                    "Password",
                    "2026-05-28 14:43",
                    vec![
                        attribute("service", "figma.com"),
                        attribute("username", "higor.design"),
                        attribute("application", "workspace"),
                    ],
                ),
                demo_item(
                    collection.id.as_str(),
                    "item-sentry",
                    "Sentry Admin",
                    "Token",
                    "2026-05-30 18:05",
                    vec![
                        attribute("service", "sentry.io"),
                        attribute("token", "ops-session-token"),
                        attribute("host", "prod-eu"),
                    ],
                ),
            ],
            "vault-infra" => vec![
                demo_item(
                    collection.id.as_str(),
                    "item-db",
                    "Postgres Cluster",
                    "Password",
                    "2026-06-04 07:55",
                    vec![
                        attribute("service", "postgres"),
                        attribute("user", "vault_admin"),
                        attribute("host", "db.internal"),
                    ],
                ),
                demo_item(
                    collection.id.as_str(),
                    "item-k8s",
                    "Kubernetes API",
                    "Token",
                    "2026-06-03 12:08",
                    vec![
                        attribute("service", "kubernetes"),
                        attribute("tokenkey", "cluster-access"),
                        attribute("application", "ops"),
                    ],
                ),
            ],
            _ => Vec::new(),
        })
    }

    async fn get_secret(
        &self,
        _collection: &SecretCollection,
        item: &SecretItem,
    ) -> Result<String> {
        let secret = match item.item_key.as_str() {
            "item-github" => "ghp_demo_personal_access_token",
            "item-mail" => "proton-demo-passphrase-42",
            "item-figma" => "figma-demo-workspace-key",
            "item-sentry" => "sntr_demo_admin_token",
            "item-db" => "postgres-demo-supersecret",
            "item-k8s" => "k8s-demo-bearer-token",
            _ => return Err(eyre!("demo secret not found")),
        };

        Ok(String::from(secret))
    }
}

fn demo_item(
    collection_id: &str,
    item_key: &str,
    name: &str,
    kind: &str,
    updated_at: &str,
    attributes: Vec<SecretAttribute>,
) -> SecretItem {
    SecretItem {
        collection_id: String::from(collection_id),
        item_key: String::from(item_key),
        name: String::from(name),
        kind: String::from(kind),
        source: String::from("Demo Source"),
        updated_at: String::from(updated_at),
        secret_preview: String::from(HIDDEN_SECRET),
        is_secret_visible: false,
        attributes,
    }
}

fn attribute(key: &str, value: &str) -> SecretAttribute {
    SecretAttribute {
        key: String::from(key),
        value: String::from(value),
    }
}
