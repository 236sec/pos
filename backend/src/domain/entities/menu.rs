use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MenuItem {
    pub id: Uuid,
    pub name: String,
    pub name_en: String,
    pub category_id: Uuid,
    pub category_name: Option<String>,
    pub base_price: i64,
    pub image_url: Option<String>,
    pub is_available: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MenuCategory {
    pub id: Uuid,
    pub name: String,
    pub name_en: String,
    pub sort_order: i32,
    pub item_count: Option<i64>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ModifierGroup {
    pub id: Uuid,
    pub menu_item_id: Uuid,
    pub name: String,
    pub name_en: String,
    pub selection_type: String,
    pub is_required: bool,
    pub sort_order: i32,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ModifierOption {
    pub id: Uuid,
    pub modifier_group_id: Uuid,
    pub name: String,
    pub name_en: String,
    pub price: i64,
    pub sort_order: i32,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChannelPrice {
    pub id: Uuid,
    pub menu_item_id: Uuid,
    pub channel: String,
    pub price: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
