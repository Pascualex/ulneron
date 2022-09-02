use uuid::Uuid;

use crate::protocol::data::Action;

#[derive(Default)]
pub struct ControllersInfo {
    pub vec: Vec<ControllerInfo>,
}

pub struct ControllerInfo {
    pub uuid: Uuid,
    pub action: Action,
}

impl ControllerInfo {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid,
            action: Action::new(),
        }
    }
}
