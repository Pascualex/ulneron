use bevy::prelude::*;

use crate::{
    client::game::resources::{LocalPlayer, PlayersInfo},
    protocol::{
        data::{Action, UpstreamData},
        events::UpstreamEvent,
    },
};

pub fn upstream_writer(
    local_player: Res<LocalPlayer>,
    players_info: Res<PlayersInfo>,
    input: Res<Input<KeyCode>>,
    mut writer: EventWriter<UpstreamEvent>,
) {
    if players_info.vec.is_empty() {
        let data = UpstreamData::Join(local_player.uuid);
        writer.send(UpstreamEvent::new_local(data));
        return;
    }

    let mut action = Action::new();

    if input.pressed(KeyCode::Up) {
        action.direction.y += 1.0;
    }
    if input.pressed(KeyCode::Right) {
        action.direction.x += 1.0;
    }
    if input.pressed(KeyCode::Down) {
        action.direction.y -= 1.0;
    }
    if input.pressed(KeyCode::Left) {
        action.direction.x -= 1.0;
    }

    let data = UpstreamData::Action(action);
    writer.send(UpstreamEvent::new_local(data));
}
