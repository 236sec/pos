use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: Uuid,
    pub branch_id: Uuid,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}
