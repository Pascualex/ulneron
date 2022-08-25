use uuid::Uuid;

pub struct LocalPlayer {
    pub id: Uuid,
}

impl Default for LocalPlayer {
    fn default() -> Self {
        Self { id: Uuid::new_v4() }
    }
}
