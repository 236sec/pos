use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::application::app_error::{AppError, AppResult};
use crate::domain::entities::menu::{ChannelPrice, MenuCategory, MenuItem, ModifierGroup, ModifierOption};

// ── Input types ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateItemInput {
    pub name: String,
    pub name_en: String,
    pub category_id: Uuid,
    pub base_price: i64,
    pub image_url: Option<String>,
    pub modifier_groups: Vec<ModifierGroupInput>,
    pub channel_prices: Vec<ChannelPriceInput>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemInput {
    pub name: String,
    pub name_en: String,
    pub category_id: Uuid,
    pub base_price: i64,
    pub image_url: Option<String>,
    pub modifier_groups: Vec<ModifierGroupInput>,
    pub channel_prices: Vec<ChannelPriceInput>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierGroupInput {
    pub name: String,
    pub name_en: String,
    pub selection_type: String,
    pub is_required: bool,
    pub sort_order: i32,
    pub options: Vec<ModifierOptionInput>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierOptionInput {
    pub name: String,
    pub name_en: String,
    pub price: i64,
    pub sort_order: i32,
}

#[derive(Debug, Deserialize)]
pub struct ChannelPriceInput {
    pub channel: String,
    pub price: i64,
}

// ── Response types ────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct MenuItemDetail {
    pub id: Uuid,
    pub name: String,
    pub name_en: String,
    pub category_id: Uuid,
    pub category_name: Option<String>,
    pub base_price: i64,
    pub image_url: Option<String>,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub modifier_groups: Vec<ModifierGroupDetail>,
    pub channel_prices: Vec<ChannelPrice>,
}

#[derive(Debug, Serialize)]
pub struct ModifierGroupDetail {
    pub id: Uuid,
    pub menu_item_id: Uuid,
    pub name: String,
    pub name_en: String,
    pub selection_type: String,
    pub is_required: bool,
    pub sort_order: i32,
    pub options: Vec<ModifierOption>,
}

// ── Persistence trait ─────────────────────────────────────────────────────

#[async_trait]
pub trait MenuPersistence: Send + Sync {
    // Items
    async fn create_item(&self, input: &CreateItemInput) -> AppResult<MenuItem>;
    async fn get_all_items(&self) -> AppResult<Vec<MenuItem>>;
    async fn get_item_by_id(&self, id: Uuid) -> AppResult<Option<MenuItem>>;
    async fn update_item(&self, id: Uuid, input: &UpdateItemInput) -> AppResult<MenuItem>;
    async fn soft_delete_item(&self, id: Uuid) -> AppResult<()>;

    // Categories
    async fn get_all_categories(&self) -> AppResult<Vec<MenuCategory>>;
    async fn create_category(
        &self,
        name: &str,
        name_en: &str,
        sort_order: i32,
    ) -> AppResult<MenuCategory>;
    async fn update_category(
        &self,
        id: Uuid,
        name: &str,
        name_en: &str,
        sort_order: i32,
    ) -> AppResult<MenuCategory>;
    async fn soft_delete_category(&self, id: Uuid) -> AppResult<()>;

    // Modifier groups
    async fn get_modifier_groups_for_item(&self, item_id: Uuid) -> AppResult<Vec<ModifierGroup>>;
    async fn get_modifier_options(&self, group_id: Uuid) -> AppResult<Vec<ModifierOption>>;
    async fn create_modifier_group(
        &self,
        item_id: Uuid,
        name: &str,
        name_en: &str,
        selection_type: &str,
        is_required: bool,
        sort_order: i32,
        options: &[ModifierOptionInput],
    ) -> AppResult<ModifierGroup>;
    async fn update_modifier_group(
        &self,
        id: Uuid,
        name: &str,
        name_en: &str,
        selection_type: &str,
        is_required: bool,
        sort_order: i32,
        options: &[ModifierOptionInput],
    ) -> AppResult<ModifierGroup>;
    async fn soft_delete_modifier_group(&self, id: Uuid) -> AppResult<()>;

    // Channel prices
    async fn get_channel_prices_for_item(&self, item_id: Uuid) -> AppResult<Vec<ChannelPrice>>;
    async fn upsert_channel_prices(
        &self,
        item_id: Uuid,
        prices: &[ChannelPriceInput],
    ) -> AppResult<()>;
}

// ── Use cases ─────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct MenuUseCases {
    persistence: Arc<dyn MenuPersistence>,
}

impl MenuUseCases {
    pub fn new(persistence: Arc<dyn MenuPersistence>) -> Self {
        Self { persistence }
    }

    // ── Items ──

    #[instrument(skip(self))]
    pub async fn create_item(&self, input: &CreateItemInput) -> AppResult<MenuItem> {
        info!("Creating menu item: {}", input.name);
        let item = self.persistence.create_item(input).await?;
        info!("Menu item created: {}", item.id);
        Ok(item)
    }

    #[instrument(skip(self))]
    pub async fn get_all_items(&self) -> AppResult<Vec<MenuItem>> {
        info!("Fetching all menu items");
        self.persistence.get_all_items().await
    }

    #[instrument(skip(self))]
    pub async fn get_item_by_id(&self, id: Uuid) -> AppResult<Option<MenuItem>> {
        info!("Fetching menu item: {}", id);
        self.persistence.get_item_by_id(id).await
    }

    #[instrument(skip(self))]
    pub async fn get_item_detail(&self, id: Uuid) -> AppResult<MenuItemDetail> {
        info!("Fetching menu item detail: {}", id);

        let item = self
            .persistence
            .get_item_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Menu item {} not found", id)))?;

        let groups = self
            .persistence
            .get_modifier_groups_for_item(id)
            .await?;

        let mut modifier_groups = Vec::with_capacity(groups.len());
        for group in groups {
            let options = self.persistence.get_modifier_options(group.id).await?;
            modifier_groups.push(ModifierGroupDetail {
                id: group.id,
                menu_item_id: group.menu_item_id,
                name: group.name,
                name_en: group.name_en,
                selection_type: group.selection_type,
                is_required: group.is_required,
                sort_order: group.sort_order,
                options,
            });
        }

        let channel_prices = self
            .persistence
            .get_channel_prices_for_item(id)
            .await?;

        Ok(MenuItemDetail {
            id: item.id,
            name: item.name,
            name_en: item.name_en,
            category_id: item.category_id,
            category_name: item.category_name,
            base_price: item.base_price,
            image_url: item.image_url,
            is_available: item.is_available,
            created_at: item.created_at,
            updated_at: item.updated_at,
            modifier_groups,
            channel_prices,
        })
    }

    #[instrument(skip(self))]
    pub async fn update_item(&self, id: Uuid, input: &UpdateItemInput) -> AppResult<MenuItem> {
        info!("Updating menu item: {}", id);
        let item = self.persistence.update_item(id, input).await?;
        info!("Menu item updated: {}", id);
        Ok(item)
    }

    #[instrument(skip(self))]
    pub async fn soft_delete_item(&self, id: Uuid) -> AppResult<()> {
        info!("Soft deleting menu item: {}", id);
        self.persistence.soft_delete_item(id).await?;
        info!("Menu item soft deleted: {}", id);
        Ok(())
    }

    // ── Categories ──

    #[instrument(skip(self))]
    pub async fn get_all_categories(&self) -> AppResult<Vec<MenuCategory>> {
        info!("Fetching all menu categories");
        self.persistence.get_all_categories().await
    }

    #[instrument(skip(self))]
    pub async fn create_category(
        &self,
        name: &str,
        name_en: &str,
        sort_order: i32,
    ) -> AppResult<MenuCategory> {
        info!("Creating menu category: {}", name);
        let cat = self
            .persistence
            .create_category(name, name_en, sort_order)
            .await?;
        info!("Menu category created: {}", cat.id);
        Ok(cat)
    }

    #[instrument(skip(self))]
    pub async fn update_category(
        &self,
        id: Uuid,
        name: &str,
        name_en: &str,
        sort_order: i32,
    ) -> AppResult<MenuCategory> {
        info!("Updating menu category: {}", id);
        let cat = self
            .persistence
            .update_category(id, name, name_en, sort_order)
            .await?;
        info!("Menu category updated: {}", id);
        Ok(cat)
    }

    #[instrument(skip(self))]
    pub async fn soft_delete_category(&self, id: Uuid) -> AppResult<()> {
        info!("Soft deleting menu category: {}", id);
        self.persistence.soft_delete_category(id).await?;
        info!("Menu category soft deleted: {}", id);
        Ok(())
    }

    // ── Modifier groups ──

    #[instrument(skip(self))]
    pub async fn get_modifier_groups_for_item(
        &self,
        item_id: Uuid,
    ) -> AppResult<Vec<ModifierGroup>> {
        info!("Fetching modifier groups for item: {}", item_id);
        self.persistence.get_modifier_groups_for_item(item_id).await
    }

    #[instrument(skip(self))]
    pub async fn create_modifier_group(
        &self,
        item_id: Uuid,
        name: &str,
        name_en: &str,
        selection_type: &str,
        is_required: bool,
        sort_order: i32,
        options: &[ModifierOptionInput],
    ) -> AppResult<ModifierGroup> {
        info!("Creating modifier group: {}", name);
        let group = self
            .persistence
            .create_modifier_group(
                item_id,
                name,
                name_en,
                selection_type,
                is_required,
                sort_order,
                options,
            )
            .await?;
        info!("Modifier group created: {}", group.id);
        Ok(group)
    }

    #[instrument(skip(self))]
    pub async fn update_modifier_group(
        &self,
        id: Uuid,
        name: &str,
        name_en: &str,
        selection_type: &str,
        is_required: bool,
        sort_order: i32,
        options: &[ModifierOptionInput],
    ) -> AppResult<ModifierGroup> {
        info!("Updating modifier group: {}", id);
        let group = self
            .persistence
            .update_modifier_group(id, name, name_en, selection_type, is_required, sort_order, options)
            .await?;
        info!("Modifier group updated: {}", id);
        Ok(group)
    }

    #[instrument(skip(self))]
    pub async fn soft_delete_modifier_group(&self, id: Uuid) -> AppResult<()> {
        info!("Soft deleting modifier group: {}", id);
        self.persistence.soft_delete_modifier_group(id).await?;
        info!("Modifier group soft deleted: {}", id);
        Ok(())
    }

    // ── Channel prices ──

    #[instrument(skip(self))]
    pub async fn get_channel_prices_for_item(
        &self,
        item_id: Uuid,
    ) -> AppResult<Vec<ChannelPrice>> {
        info!("Fetching channel prices for item: {}", item_id);
        self.persistence.get_channel_prices_for_item(item_id).await
    }

    #[instrument(skip(self))]
    pub async fn upsert_channel_prices(
        &self,
        item_id: Uuid,
        prices: &[ChannelPriceInput],
    ) -> AppResult<()> {
        info!("Upserting channel prices for item: {}", item_id);
        self.persistence.upsert_channel_prices(item_id, prices).await?;
        info!("Channel prices upserted for item: {}", item_id);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    struct MockMenuPersistence;

    #[async_trait]
    impl MenuPersistence for MockMenuPersistence {
        async fn create_item(&self, _input: &CreateItemInput) -> AppResult<MenuItem> {
            Ok(MenuItem {
                id: Uuid::new_v4(),
                name: _input.name.clone(),
                name_en: _input.name_en.clone(),
                category_id: _input.category_id,
                category_name: None,
                base_price: _input.base_price,
                image_url: _input.image_url.clone(),
                is_available: true,
                deleted_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn get_all_items(&self) -> AppResult<Vec<MenuItem>> {
            Ok(vec![])
        }

        async fn get_item_by_id(&self, _id: Uuid) -> AppResult<Option<MenuItem>> {
            Ok(None)
        }

        async fn update_item(&self, _id: Uuid, _input: &UpdateItemInput) -> AppResult<MenuItem> {
            Ok(MenuItem {
                id: _id,
                name: _input.name.clone(),
                name_en: _input.name_en.clone(),
                category_id: _input.category_id,
                category_name: None,
                base_price: _input.base_price,
                image_url: _input.image_url.clone(),
                is_available: true,
                deleted_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn soft_delete_item(&self, _id: Uuid) -> AppResult<()> {
            Ok(())
        }

        async fn get_all_categories(&self) -> AppResult<Vec<MenuCategory>> {
            Ok(vec![])
        }

        async fn create_category(
            &self,
            name: &str,
            name_en: &str,
            sort_order: i32,
        ) -> AppResult<MenuCategory> {
            Ok(MenuCategory {
                id: Uuid::new_v4(),
                name: name.to_string(),
                name_en: name_en.to_string(),
                sort_order,
                item_count: None,
                deleted_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn update_category(
            &self,
            id: Uuid,
            name: &str,
            name_en: &str,
            sort_order: i32,
        ) -> AppResult<MenuCategory> {
            Ok(MenuCategory {
                id,
                name: name.to_string(),
                name_en: name_en.to_string(),
                sort_order,
                item_count: None,
                deleted_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn soft_delete_category(&self, _id: Uuid) -> AppResult<()> {
            Ok(())
        }

        async fn get_modifier_groups_for_item(
            &self,
            _item_id: Uuid,
        ) -> AppResult<Vec<ModifierGroup>> {
            Ok(vec![])
        }

        async fn get_modifier_options(&self, _group_id: Uuid) -> AppResult<Vec<ModifierOption>> {
            Ok(vec![])
        }

        async fn create_modifier_group(
            &self,
            item_id: Uuid,
            name: &str,
            name_en: &str,
            selection_type: &str,
            is_required: bool,
            sort_order: i32,
            _options: &[ModifierOptionInput],
        ) -> AppResult<ModifierGroup> {
            Ok(ModifierGroup {
                id: Uuid::new_v4(),
                menu_item_id: item_id,
                name: name.to_string(),
                name_en: name_en.to_string(),
                selection_type: selection_type.to_string(),
                is_required,
                sort_order,
                deleted_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn update_modifier_group(
            &self,
            id: Uuid,
            name: &str,
            name_en: &str,
            selection_type: &str,
            is_required: bool,
            sort_order: i32,
            _options: &[ModifierOptionInput],
        ) -> AppResult<ModifierGroup> {
            Ok(ModifierGroup {
                id,
                menu_item_id: Uuid::new_v4(),
                name: name.to_string(),
                name_en: name_en.to_string(),
                selection_type: selection_type.to_string(),
                is_required,
                sort_order,
                deleted_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn soft_delete_modifier_group(&self, _id: Uuid) -> AppResult<()> {
            Ok(())
        }

        async fn get_channel_prices_for_item(
            &self,
            _item_id: Uuid,
        ) -> AppResult<Vec<ChannelPrice>> {
            Ok(vec![])
        }

        async fn upsert_channel_prices(
            &self,
            _item_id: Uuid,
            _prices: &[ChannelPriceInput],
        ) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn create_item_works() {
        let use_cases = MenuUseCases::new(Arc::new(MockMenuPersistence));

        let input = CreateItemInput {
            name: "Espresso".into(),
            name_en: "Espresso".into(),
            category_id: Uuid::new_v4(),
            base_price: 3000,
            image_url: None,
            modifier_groups: vec![],
            channel_prices: vec![],
        };

        let result = use_cases.create_item(&input).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Espresso");
    }

    #[tokio::test]
    async fn get_all_items_works() {
        let use_cases = MenuUseCases::new(Arc::new(MockMenuPersistence));

        let result = use_cases.get_all_items().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_item_works() {
        let use_cases = MenuUseCases::new(Arc::new(MockMenuPersistence));

        let result = use_cases.soft_delete_item(Uuid::new_v4()).await;

        assert!(result.is_ok());
    }
}
