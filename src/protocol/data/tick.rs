use bevy::utils::HashMap;
use uuid::Uuid;

use crate::protocol::data::Action;

pub type Tick = HashMap<Uuid, Action>;
