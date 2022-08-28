use bevy::prelude::*;

use crate::client::{controller::resources::ControllerInfo, game::resources::PlayersInfo};

pub fn join_detection(players_info: Res<PlayersInfo>, mut controller_info: ResMut<ControllerInfo>) {
    if !players_info.is_changed() {
        return;
    }

    controller_info.id = players_info
        .uuids
        .iter()
        .enumerate()
        .find(|(_, uuid)| **uuid == controller_info.uuid)
        .map(|(id, _)| id);
}
