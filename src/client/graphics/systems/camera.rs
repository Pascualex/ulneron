use bevy::prelude::*;

use crate::client::{controller::resources::ControllerInfo, game::resources::PlayersInfo};

pub fn camera(
    controller_info: Res<ControllerInfo>,
    players_info: Res<PlayersInfo>,
    player_query: Query<&Transform, Without<Camera>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let controller_uuid = controller_info.uuid;
    if let Some(player_info) = players_info.vec.iter().find(|i| i.uuid == controller_uuid) {
        let player_transform = player_query.get(player_info.entity).unwrap();
        let mut camera_transform = camera_query.single_mut();
        let position = player_transform.translation;
        camera_transform.translation = Vec3::new(position.x - 5.0, 15.0, position.z);
    }
}
