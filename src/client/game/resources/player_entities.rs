use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct PlayerEntities {
    pub vec: Vec<Entity>,
}
