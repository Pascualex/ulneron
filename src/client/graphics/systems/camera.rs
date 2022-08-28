use bevy::prelude::*;

use crate::client::game::resources::{LocalPlayer, PlayersInfo};

pub fn camera(
    local_player: Res<LocalPlayer>,
    players_info: Res<PlayersInfo>,
    player_query: Query<&Transform, Without<Camera>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let local_uuid = local_player.uuid;
    if let Some(player_info) = players_info.vec.iter().find(|i| i.uuid == local_uuid) {
        let player_transform = player_query.get(player_info.entity).unwrap();
        let mut camera_transform = camera_query.single_mut();
        let position = player_transform.translation;
        camera_transform.translation = Vec3::new(position.x - 5.0, 15.0, position.z);
    }
}
