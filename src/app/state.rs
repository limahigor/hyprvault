use color_eyre::Result;

use crate::data::SecretSource;

#[derive(Clone, Debug)]
pub struct SecretItem {
    pub name: String,
    pub kind: String,
    pub source: String,
    pub updated_at: String,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct App {
    items: Vec<SecretItem>,
    selected_index: usize,
}

impl App {
    pub async fn new(source: &dyn SecretSource) -> Result<Self> {
        let items = source.load_items().await?;

        Ok(Self {
            items,
            selected_index: 0,
        })
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        self.selected_index = (self.selected_index + 1) % self.items.len();
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        if self.selected_index == 0 {
            self.selected_index = self.items.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn items(&self) -> &[SecretItem] {
        &self.items
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn selected_item(&self) -> Option<&SecretItem> {
        self.items.get(self.selected_index)
    }
}
