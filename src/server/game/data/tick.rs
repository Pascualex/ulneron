use serde::{Deserialize, Serialize};

use crate::client::controller::data::Action;

#[derive(Clone, Serialize, Deserialize)]
pub struct Tick {
    pub actions: Vec<Action>,
}

impl Tick {
    pub fn new(actions: Vec<Action>) -> Self {
        Self { actions }
    }
}
