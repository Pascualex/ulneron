use bevy::prelude::*;

use crate::{
    events::upstream::InputEvent,
    server::{components::*, resources::EntitiesIds},
};

pub fn movement_input(
    mut input_reader: EventReader<InputEvent>,
    entities_ids: Res<EntitiesIds>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for input in input_reader.iter() {
        let entity = match entities_ids.map.get(&input.id) {
            Some(entity) => *entity,
            None => continue,
        };
        let mut velocity = match query.get_mut(entity) {
            Ok(result) => result,
            Err(_) => continue,
        };
        velocity.value = input.direction * 5.0;
    }
}
