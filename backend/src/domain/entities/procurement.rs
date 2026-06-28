use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PurchaseOrder {
    pub id: Uuid,
    pub supplier_id: Uuid,
    pub status: String,
}
