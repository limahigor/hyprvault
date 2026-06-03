use async_trait::async_trait;
use color_eyre::Result;

use crate::app::state::SecretItem;

#[async_trait]
pub trait SecretSource {
    async fn load_items(&self) -> Result<Vec<SecretItem>>;
}
