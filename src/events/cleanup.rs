use bevy::prelude::*;

use crate::events::{downstream::*, upstream::*};

pub fn cleanup(
    // upstream events
    mut input_events: ResMut<Events<InputEvent>>,
    mut join_events: ResMut<Events<JoinEvent>>,
    // downstream events
    mut movement_events: ResMut<Events<MovementEvent>>,
    mut spawn_events: ResMut<Events<SpawnEvent>>,
) {
    // upstream events
    input_events.update();
    join_events.update();
    // downstream events
    movement_events.update();
    spawn_events.update();
}
