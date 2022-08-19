use bevy::prelude::*;

use crate::events::{downstream::*, upstream::*};

#[derive(Default)]
pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app
            // upstream events
            .init_resource::<Events<InputEvent>>()
            .init_resource::<Events<JoinEvent>>()
            // downstream events
            .init_resource::<Events<MovementEvent>>();
    }
}
