use bevy::prelude::*;

use crate::protocol::{data::Action, events::UpstreamEvent};

pub fn setup(mut upstream_writer: EventWriter<UpstreamEvent>) {
    upstream_writer.send(UpstreamEvent::new_local(Action::new()));
}
