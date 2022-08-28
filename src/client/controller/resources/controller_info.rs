use uuid::Uuid;

pub struct ControllerInfo {
    pub uuid: Uuid,
    pub id: Option<usize>,
}

impl Default for ControllerInfo {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            id: None,
        }
    }
}
