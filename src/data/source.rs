use async_trait::async_trait;
use color_eyre::Result;

use crate::app::state::{SecretCollection, SecretItem};

#[async_trait]
pub trait SecretSource {
    async fn load_collections(&self) -> Result<Vec<SecretCollection>>;
    async fn load_items(&self, collection: &SecretCollection) -> Result<Vec<SecretItem>>;
}
