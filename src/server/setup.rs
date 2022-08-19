use bevy::prelude::*;

use crate::server::components::*;

pub fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Position::from_xy(0.0, 0.0))
        .insert(Velocity::from_xy(0.0, 0.0))
        .insert(Player);
}
