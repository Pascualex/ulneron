use bevy::{prelude::*, utils::HashMap};
use uuid::Uuid;

#[derive(Default)]
pub struct Buffer {
    pub directions: HashMap<Uuid, Vec2>,
}
