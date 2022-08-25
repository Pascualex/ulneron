use bevy::utils::HashMap;

use crate::protocol::data::Action;

pub type Tick = HashMap<u32, Action>;
