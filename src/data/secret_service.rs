use async_trait::async_trait;
use chrono::{DateTime, Local};
use color_eyre::Result;
use secret_service::{EncryptionType, SecretService};
use zbus::zvariant::OwnedObjectPath;

use crate::{
    app::state::{SecretCollection, SecretItem},
    data::source::SecretSource,
};

#[derive(Debug, Default)]
pub struct SecretServiceSource;

#[async_trait]
impl SecretSource for SecretServiceSource {
    async fn load_collections(&self) -> Result<Vec<SecretCollection>> {
        let mut collections = Vec::new();

        let service = SecretService::connect(EncryptionType::Dh).await?;
        let secret_collections = service.get_all_collections().await?;

        for secret_collection in secret_collections {
            let items = secret_collection.get_all_items().await?;

            if items.is_empty() {
                continue;
            }

            let secret_key = secret_collection.collection_path.to_string();
            let collection = SecretCollection {
                id: secret_key.clone(),
                name: secret_collection.get_label().await?,
                secret_key,
            };

            collections.push(collection);
        }

        Ok(collections)
    }

    async fn load_items(&self, collection: &SecretCollection) -> Result<Vec<SecretItem>> {
        let mut items = Vec::new();

        let service = SecretService::connect(EncryptionType::Dh).await?;
        let path: OwnedObjectPath = collection.secret_key.as_str().try_into()?;
        let secret_collection = service.get_collection_by_path(path).await?;
        let secret_items = secret_collection.get_all_items().await?;

        for secret_item in secret_items {
            items.push(SecretItem {
                collection_id: collection.id.clone(),
                name: secret_item.get_label().await?,
                kind: String::from("Password"),
                source: String::from("Secret Service"),
                updated_at: DateTime::from_timestamp(secret_item.get_modified().await? as i64, 0)
                    .map(|timestamp| {
                        timestamp
                            .with_timezone(&Local)
                            .format("%Y-%m-%d %H:%M")
                            .to_string()
                    })
                    .unwrap_or_else(|| String::from("unknown")),
                tags: vec![],
            });
        }

        Ok(items)
    }
}
