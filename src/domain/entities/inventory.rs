use uuid::Uuid;
use crate::domain::primitives::Quantity;

#[derive(Debug, Clone)]
pub struct StockItem {
    pub id: Uuid,
    pub ingredient_name: String,
    pub quantity: Quantity,
}
