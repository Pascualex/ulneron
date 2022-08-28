use uuid::Uuid;

pub struct LocalPlayer {
    pub uuid: Uuid,
}

impl Default for LocalPlayer {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}
