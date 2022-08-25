use bevy::prelude::*;

use crate::{
    client::resources::LocalPlayer,
    protocol::{data::Action, events::UpstreamEvent},
};

pub fn upstream_writer(
    input: Res<Input<KeyCode>>,
    local_player: Res<LocalPlayer>,
    mut writer: EventWriter<UpstreamEvent>,
) {
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

    writer.send(UpstreamEvent::new(local_player.id, action));
}
