use bevy::prelude::*;

use crate::client::{controller::resources::ControllerInfo, game::resources::PlayersInfo};

pub fn camera(
    controller_info: Res<ControllerInfo>,
    players_info: Res<PlayersInfo>,
    player_query: Query<&Transform, Without<Camera>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let player_id = match controller_info.id {
        Some(player_id) => player_id,
        None => return,
    };
    let player_entity = match players_info.entities.get(player_id) {
        Some(player_entity) => *player_entity,
        None => return,
    };
    let player_transform = player_query.get(player_entity).unwrap();
    let mut camera_transform = camera_query.single_mut();
    let position = player_transform.translation;
    camera_transform.translation = Vec3::new(position.x - 5.0, 15.0, position.z);
}
