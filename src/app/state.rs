use color_eyre::{Result, eyre::eyre};

use crate::data::SecretSource;

#[derive(Clone, Debug, Default)]
pub struct SecretCollection {
    pub id: String,
    pub name: String,
    pub secret_key: String,
}

#[derive(Clone, Debug, Default)]
pub struct SecretItem {
    pub collection_id: String,
    pub name: String,
    pub kind: String,
    pub source: String,
    pub updated_at: String,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct App {
    collections: Vec<SecretCollection>,
    items: Vec<SecretItem>,
    selected_collection_index: usize,
    selected_item_index: usize,
}

impl App {
    pub async fn new(source: &dyn SecretSource) -> Result<Self> {
        let collections = source.load_collections().await?;

        if collections.is_empty() {
            return Err(eyre!("secret source returned no collections"));
        }

        let first_collection = &collections[0];
        let items = source.load_items(first_collection).await?;

        Ok(Self {
            collections,
            items,
            selected_collection_index: 0,
            selected_item_index: 0,
        })
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        self.selected_item_index = (self.selected_item_index + 1) % self.items.len();
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        if self.selected_item_index == 0 {
            self.selected_item_index = self.items.len() - 1;
        } else {
            self.selected_item_index -= 1;
        }
    }

    pub async fn reload_items(&mut self, source: &dyn SecretSource) -> Result<()> {
        let collection = self
            .selected_collection()
            .ok_or_else(|| eyre!("no collection selected"))?
            .clone();

        self.items = source.load_items(&collection).await?;

        Ok(())
    }

    pub async fn next_collection(&mut self, source: &dyn SecretSource) -> Result<()> {
        if self.collections.is_empty() {
            return Ok(());
        }

        self.selected_collection_index =
            (self.selected_collection_index + 1) % self.collections.len();
        self.selected_item_index = 0;

        self.reload_items(source).await
    }

    pub async fn previous_collection(&mut self, source: &dyn SecretSource) -> Result<()> {
        if self.collections.is_empty() {
            return Ok(());
        }

        if self.selected_collection_index == 0 {
            self.selected_collection_index = self.collections.len() - 1;
        } else {
            self.selected_collection_index -= 1;
        }

        self.selected_item_index = 0;

        self.reload_items(source).await
    }

    pub fn collections(&self) -> &[SecretCollection] {
        &self.collections
    }

    pub fn filtered_items(&self) -> Vec<&SecretItem> {
        let Some(collection) = self.selected_collection() else {
            return Vec::new();
        };

        self.items
            .iter()
            .filter(|item| item.collection_id == collection.id)
            .collect()
    }

    pub fn selected_collection_index(&self) -> usize {
        self.selected_collection_index
    }

    pub fn selected_collection(&self) -> Option<&SecretCollection> {
        self.collections.get(self.selected_collection_index)
    }

    pub fn selected_item_index(&self) -> usize {
        self.selected_item_index
    }

    pub fn selected_item(&self) -> Option<&SecretItem> {
        self.filtered_items().get(self.selected_item_index).copied()
    }
}
