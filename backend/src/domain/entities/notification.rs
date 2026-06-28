use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: Uuid,
    pub message: String,
    pub created_at: chrono::NaiveDateTime,
}
