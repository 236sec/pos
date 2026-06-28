use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub id: Uuid,
    pub name: String,
}
