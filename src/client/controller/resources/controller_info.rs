use uuid::Uuid;

pub struct ControllerInfo {
    pub uuid: Uuid,
}

impl Default for ControllerInfo {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}
