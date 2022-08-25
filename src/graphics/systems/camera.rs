use bevy::prelude::*;

use crate::client::{components::Player, resources::LocalPlayer};

pub fn camera(
    player_query: Query<(&Player, &Transform), Without<Camera>>,
    local_player: Res<LocalPlayer>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Some((_, transform)) = player_query.iter().find(|(p, _)| p.id == local_player.id) {
        let mut camera_transform = camera_query.single_mut();
        let position = transform.translation;
        camera_transform.translation = Vec3::new(position.x - 5.0, 15.0, position.z);
    }
}
