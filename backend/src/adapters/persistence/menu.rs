use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    application::{
        app_error::{AppError, AppResult},
        use_cases::menu::{
            ChannelPriceInput, CreateItemInput, MenuPersistence, ModifierOptionInput,
            UpdateItemInput,
        },
    },
    domain::entities::menu::{ChannelPrice, MenuCategory, MenuItem, ModifierGroup, ModifierOption},
};

#[async_trait]
impl MenuPersistence for PostgresPersistence {
    // ── Items ──

    async fn create_item(&self, input: &CreateItemInput) -> AppResult<MenuItem> {
        let mut tx = self.pool.begin().await?;

        let item_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO menu_items (id, name, name_en, category_id, base_price, image_url, is_available, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, true, $7, $7)
            "#,
        )
        .bind(item_id)
        .bind(&input.name)
        .bind(&input.name_en)
        .bind(input.category_id)
        .bind(input.base_price)
        .bind(&input.image_url)
        .bind(now)
        .execute(&mut *tx)
        .await?;

        // Insert modifier groups with options
        for group_input in &input.modifier_groups {
            let group_id = Uuid::new_v4();
            sqlx::query(
                r#"
                INSERT INTO modifier_groups (id, menu_item_id, name, name_en, selection_type, is_required, sort_order, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8)
                "#,
            )
            .bind(group_id)
            .bind(item_id)
            .bind(&group_input.name)
            .bind(&group_input.name_en)
            .bind(&group_input.selection_type)
            .bind(group_input.is_required)
            .bind(group_input.sort_order)
            .bind(now)
            .execute(&mut *tx)
            .await?;

            for opt_input in &group_input.options {
                let opt_id = Uuid::new_v4();
                sqlx::query(
                    r#"
                    INSERT INTO modifier_options (id, modifier_group_id, name, name_en, price, sort_order, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $7)
                    "#,
                )
                .bind(opt_id)
                .bind(group_id)
                .bind(&opt_input.name)
                .bind(&opt_input.name_en)
                .bind(opt_input.price)
                .bind(opt_input.sort_order)
                .bind(now)
                .execute(&mut *tx)
                .await?;
            }
        }

        // Insert channel prices
        for price_input in &input.channel_prices {
            let price_id = Uuid::new_v4();
            sqlx::query(
                r#"
                INSERT INTO channel_prices (id, menu_item_id, channel, price, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $5)
                "#,
            )
            .bind(price_id)
            .bind(item_id)
            .bind(&price_input.channel)
            .bind(price_input.price)
            .bind(now)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        // Fetch the freshly created item with category_name = NULL
        let item = self.get_item_by_id(item_id).await?.ok_or_else(|| {
            AppError::Internal("Created item not found after insert".into())
        })?;

        Ok(item)
    }

    async fn get_all_items(&self) -> AppResult<Vec<MenuItem>> {
        let rows = sqlx::query_as::<_, MenuItem>(
            r#"
            SELECT mi.id, mi.name, mi.name_en, mi.category_id, mc.name AS category_name,
                   mi.base_price, mi.image_url, mi.is_available,
                   mi.deleted_at, mi.created_at, mi.updated_at
            FROM menu_items mi
            LEFT JOIN menu_categories mc ON mc.id = mi.category_id
            WHERE mi.deleted_at IS NULL
            ORDER BY mi.created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn get_item_by_id(&self, id: Uuid) -> AppResult<Option<MenuItem>> {
        let row = sqlx::query_as::<_, MenuItem>(
            r#"
            SELECT mi.id, mi.name, mi.name_en, mi.category_id, NULL::text AS category_name,
                   mi.base_price, mi.image_url, mi.is_available,
                   mi.deleted_at, mi.created_at, mi.updated_at
            FROM menu_items mi
            WHERE mi.id = $1 AND mi.deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    async fn update_item(&self, id: Uuid, input: &UpdateItemInput) -> AppResult<MenuItem> {
        let mut tx = self.pool.begin().await?;
        let now = Utc::now();

        // Check item exists
        let existing = sqlx::query_scalar::<_, i64>("SELECT 1 FROM menu_items WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?;

        if existing.is_none() {
            return Err(AppError::NotFound(format!("Menu item {} not found", id)));
        }

        // Update item fields
        sqlx::query(
            r#"
            UPDATE menu_items
            SET name = $1, name_en = $2, category_id = $3, base_price = $4,
                image_url = $5, updated_at = $6
            WHERE id = $7
            "#,
        )
        .bind(&input.name)
        .bind(&input.name_en)
        .bind(input.category_id)
        .bind(input.base_price)
        .bind(&input.image_url)
        .bind(now)
        .bind(id)
        .execute(&mut *tx)
        .await?;

        // Hard-delete existing modifier groups (cascade deletes options)
        sqlx::query("DELETE FROM modifier_groups WHERE menu_item_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        // Re-insert modifier groups with options
        for group_input in &input.modifier_groups {
            let group_id = Uuid::new_v4();
            sqlx::query(
                r#"
                INSERT INTO modifier_groups (id, menu_item_id, name, name_en, selection_type, is_required, sort_order, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8)
                "#,
            )
            .bind(group_id)
            .bind(id)
            .bind(&group_input.name)
            .bind(&group_input.name_en)
            .bind(&group_input.selection_type)
            .bind(group_input.is_required)
            .bind(group_input.sort_order)
            .bind(now)
            .execute(&mut *tx)
            .await?;

            for opt_input in &group_input.options {
                let opt_id = Uuid::new_v4();
                sqlx::query(
                    r#"
                    INSERT INTO modifier_options (id, modifier_group_id, name, name_en, price, sort_order, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $7)
                    "#,
                )
                .bind(opt_id)
                .bind(group_id)
                .bind(&opt_input.name)
                .bind(&opt_input.name_en)
                .bind(opt_input.price)
                .bind(opt_input.sort_order)
                .bind(now)
                .execute(&mut *tx)
                .await?;
            }
        }

        // Upsert channel prices: delete existing, re-insert
        sqlx::query("DELETE FROM channel_prices WHERE menu_item_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        for price_input in &input.channel_prices {
            let price_id = Uuid::new_v4();
            sqlx::query(
                r#"
                INSERT INTO channel_prices (id, menu_item_id, channel, price, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $5)
                "#,
            )
            .bind(price_id)
            .bind(id)
            .bind(&price_input.channel)
            .bind(price_input.price)
            .bind(now)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        // Return updated item
        self.get_item_by_id(id)
            .await?
            .ok_or_else(|| AppError::Internal("Updated item not found".into()))
    }

    async fn soft_delete_item(&self, id: Uuid) -> AppResult<()> {
        let now = Utc::now();
        let rows = sqlx::query("UPDATE menu_items SET deleted_at = $1, updated_at = $1 WHERE id = $2 AND deleted_at IS NULL")
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        if rows == 0 {
            return Err(AppError::NotFound(format!("Menu item {} not found", id)));
        }

        Ok(())
    }

    // ── Categories ──

    async fn get_all_categories(&self) -> AppResult<Vec<MenuCategory>> {
        let rows = sqlx::query_as::<_, MenuCategory>(
            r#"
            SELECT mc.id, mc.name, mc.name_en, mc.sort_order,
                   COUNT(mi.id) FILTER (WHERE mi.deleted_at IS NULL)::bigint AS item_count,
                   mc.deleted_at, mc.created_at, mc.updated_at
            FROM menu_categories mc
            LEFT JOIN menu_items mi ON mi.category_id = mc.id
            WHERE mc.deleted_at IS NULL
            GROUP BY mc.id
            ORDER BY mc.sort_order ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn create_category(
        &self,
        name: &str,
        name_en: &str,
        sort_order: i32,
    ) -> AppResult<MenuCategory> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO menu_categories (id, name, name_en, sort_order, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $5)
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(name_en)
        .bind(sort_order)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(MenuCategory {
            id,
            name: name.to_string(),
            name_en: name_en.to_string(),
            sort_order,
            item_count: Some(0),
            deleted_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    async fn update_category(
        &self,
        id: Uuid,
        name: &str,
        name_en: &str,
        sort_order: i32,
    ) -> AppResult<MenuCategory> {
        let now = Utc::now();

        let rows = sqlx::query(
            r#"
            UPDATE menu_categories
            SET name = $1, name_en = $2, sort_order = $3, updated_at = $4
            WHERE id = $5 AND deleted_at IS NULL
            "#,
        )
        .bind(name)
        .bind(name_en)
        .bind(sort_order)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows == 0 {
            return Err(AppError::NotFound(format!("Menu category {} not found", id)));
        }

        Ok(MenuCategory {
            id,
            name: name.to_string(),
            name_en: name_en.to_string(),
            sort_order,
            item_count: None,
            deleted_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    async fn soft_delete_category(&self, id: Uuid) -> AppResult<()> {
        let now = Utc::now();

        // Check for active items in this category
        let item_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM menu_items WHERE category_id = $1 AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        if item_count > 0 {
            return Err(AppError::Internal(format!(
                "Cannot delete category with {} active menu items",
                item_count
            )));
        }

        let rows = sqlx::query(
            "UPDATE menu_categories SET deleted_at = $1, updated_at = $1 WHERE id = $2 AND deleted_at IS NULL",
        )
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows == 0 {
            return Err(AppError::NotFound(format!("Menu category {} not found", id)));
        }

        Ok(())
    }

    // ── Modifier groups ──

    async fn get_modifier_groups_for_item(&self, item_id: Uuid) -> AppResult<Vec<ModifierGroup>> {
        let rows = sqlx::query_as::<_, ModifierGroup>(
            r#"
            SELECT id, menu_item_id, name, name_en, selection_type, is_required, sort_order,
                   deleted_at, created_at, updated_at
            FROM modifier_groups
            WHERE menu_item_id = $1 AND deleted_at IS NULL
            ORDER BY sort_order ASC
            "#,
        )
        .bind(item_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn get_modifier_options(&self, group_id: Uuid) -> AppResult<Vec<ModifierOption>> {
        let rows = sqlx::query_as::<_, ModifierOption>(
            r#"
            SELECT id, modifier_group_id, name, name_en, price, sort_order,
                   deleted_at, created_at, updated_at
            FROM modifier_options
            WHERE modifier_group_id = $1 AND deleted_at IS NULL
            ORDER BY sort_order ASC
            "#,
        )
        .bind(group_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn create_modifier_group(
        &self,
        item_id: Uuid,
        name: &str,
        name_en: &str,
        selection_type: &str,
        is_required: bool,
        sort_order: i32,
        options: &[ModifierOptionInput],
    ) -> AppResult<ModifierGroup> {
        let mut tx = self.pool.begin().await?;
        let now = Utc::now();
        let group_id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO modifier_groups (id, menu_item_id, name, name_en, selection_type, is_required, sort_order, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8)
            "#,
        )
        .bind(group_id)
        .bind(item_id)
        .bind(name)
        .bind(name_en)
        .bind(selection_type)
        .bind(is_required)
        .bind(sort_order)
        .bind(now)
        .execute(&mut *tx)
        .await?;

        for opt_input in options {
            let opt_id = Uuid::new_v4();
            sqlx::query(
                r#"
                INSERT INTO modifier_options (id, modifier_group_id, name, name_en, price, sort_order, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $7)
                "#,
            )
            .bind(opt_id)
            .bind(group_id)
            .bind(&opt_input.name)
            .bind(&opt_input.name_en)
            .bind(opt_input.price)
            .bind(opt_input.sort_order)
            .bind(now)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(ModifierGroup {
            id: group_id,
            menu_item_id: item_id,
            name: name.to_string(),
            name_en: name_en.to_string(),
            selection_type: selection_type.to_string(),
            is_required,
            sort_order,
            deleted_at: None,
            created_at: now,
            updated_at: now,
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
        options: &[ModifierOptionInput],
    ) -> AppResult<ModifierGroup> {
        let mut tx = self.pool.begin().await?;
        let now = Utc::now();

        // Fetch current group to get menu_item_id and verify existence
        let current = sqlx::query_as::<_, (Uuid, Uuid)>(
            "SELECT id, menu_item_id FROM modifier_groups WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?;

        let (_, menu_item_id) = current
            .ok_or_else(|| AppError::NotFound(format!("Modifier group {} not found", id)))?;

        // Update group fields
        sqlx::query(
            r#"
            UPDATE modifier_groups
            SET name = $1, name_en = $2, selection_type = $3, is_required = $4, sort_order = $5, updated_at = $6
            WHERE id = $7
            "#,
        )
        .bind(name)
        .bind(name_en)
        .bind(selection_type)
        .bind(is_required)
        .bind(sort_order)
        .bind(now)
        .bind(id)
        .execute(&mut *tx)
        .await?;

        // Hard-delete existing options, re-insert
        sqlx::query("DELETE FROM modifier_options WHERE modifier_group_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        for opt_input in options {
            let opt_id = Uuid::new_v4();
            sqlx::query(
                r#"
                INSERT INTO modifier_options (id, modifier_group_id, name, name_en, price, sort_order, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $7)
                "#,
            )
            .bind(opt_id)
            .bind(id)
            .bind(&opt_input.name)
            .bind(&opt_input.name_en)
            .bind(opt_input.price)
            .bind(opt_input.sort_order)
            .bind(now)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(ModifierGroup {
            id,
            menu_item_id,
            name: name.to_string(),
            name_en: name_en.to_string(),
            selection_type: selection_type.to_string(),
            is_required,
            sort_order,
            deleted_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    async fn soft_delete_modifier_group(&self, id: Uuid) -> AppResult<()> {
        let now = Utc::now();
        let rows = sqlx::query(
            "UPDATE modifier_groups SET deleted_at = $1, updated_at = $1 WHERE id = $2 AND deleted_at IS NULL",
        )
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows == 0 {
            return Err(AppError::NotFound(format!("Modifier group {} not found", id)));
        }

        Ok(())
    }

    // ── Channel prices ──

    async fn get_channel_prices_for_item(&self, item_id: Uuid) -> AppResult<Vec<ChannelPrice>> {
        let rows = sqlx::query_as::<_, ChannelPrice>(
            r#"
            SELECT id, menu_item_id, channel, price, created_at, updated_at
            FROM channel_prices
            WHERE menu_item_id = $1
            ORDER BY channel ASC
            "#,
        )
        .bind(item_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn upsert_channel_prices(
        &self,
        item_id: Uuid,
        prices: &[ChannelPriceInput],
    ) -> AppResult<()> {
        let mut tx = self.pool.begin().await?;
        let now = Utc::now();

        // Delete existing prices, re-insert
        sqlx::query("DELETE FROM channel_prices WHERE menu_item_id = $1")
            .bind(item_id)
            .execute(&mut *tx)
            .await?;

        for price_input in prices {
            let price_id = Uuid::new_v4();
            sqlx::query(
                r#"
                INSERT INTO channel_prices (id, menu_item_id, channel, price, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $5)
                "#,
            )
            .bind(price_id)
            .bind(item_id)
            .bind(&price_input.channel)
            .bind(price_input.price)
            .bind(now)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
