use bevy::prelude::*;

use crate::{client::resources::Ticks, protocol::events::DownstreamEvent};

pub fn downstream_reader(mut reader: EventReader<DownstreamEvent>, mut ticks: ResMut<Ticks>) {
    if !ticks.vec.is_empty() {
        ticks.vec.remove(0);
    }

    for downstream in reader.iter() {
        ticks.vec.push(downstream.tick.clone());
    }
}
