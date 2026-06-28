use crate::domain::primitives::Quantity;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct StockItem {
    pub id: Uuid,
    pub ingredient_name: String,
    pub quantity: Quantity,
}
