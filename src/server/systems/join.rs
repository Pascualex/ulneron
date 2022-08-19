use bevy::prelude::*;

use crate::{
    events::{downstream::SpawnEvent, upstream::JoinEvent},
    server::{
        components::{Id, Player, Position, Velocity},
        resources::EntitiesIds,
    },
};

pub fn join(
    mut join_reader: EventReader<JoinEvent>,
    mut entities_ids: ResMut<EntitiesIds>,
    mut commands: Commands,
    mut spawn_writer: EventWriter<SpawnEvent>,
) {
    for join in join_reader.iter() {
        if entities_ids.map.contains_key(&join.id) {
            continue;
        }
        let entity = commands
            .spawn()
            .insert(Position::from_xy(0.0, 0.0))
            .insert(Velocity::from_xy(0.0, 0.0))
            .insert(Id::new(join.id))
            .insert(Player)
            .id();
        entities_ids.map.insert(join.id, entity);
        spawn_writer.send(SpawnEvent::new(join.id, Vec2::ZERO));
    }
}
