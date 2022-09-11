use bevy::prelude::*;

use crate::client::game::{
    components::{Enemy, Player, Position, Velocity},
    resources::Ticks,
};

pub fn players_attack(
    player_query: Query<(&Position, &Velocity), With<Player>>,
    enemy_query: Query<(Entity, &Position), With<Enemy>>,
    ticks: Res<Ticks>,
    mut commands: Commands,
) {
    if ticks.current.is_none() {
        return;
    }

    for (player_position, player_velocity) in player_query.iter() {
        if player_velocity.val != Vec2::ZERO {
            continue;
        }
        for (enemy_entity, enemy_position) in enemy_query.iter() {
            let distance = (enemy_position.val - player_position.val).length();
            if distance < 3.0 {
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
