use std::time::Duration;

use bevy::prelude::*;

use crate::{
    client::game::{
        components::{Enemy, Player, Position, Velocity, Weapons},
        resources::Ticks,
    },
    TICK_STEP,
};

pub fn players_attack(
    mut player_query: Query<(&Position, &Velocity, &mut Weapons), With<Player>>,
    enemy_query: Query<(Entity, &Position), With<Enemy>>,
    ticks: Res<Ticks>,
    mut commands: Commands,
) {
    if ticks.current.is_none() {
        return;
    }

    for (player_position, player_velocity, mut player_weapons) in player_query.iter_mut() {
        if player_velocity.val != Vec2::ZERO {
            continue;
        }
        player_weapons
            .timer
            .tick(Duration::from_secs_f32(TICK_STEP));
        let shot_count = player_weapons.timer.times_finished_this_tick();
        if shot_count == 0 {
            continue;
        }
        let mut enemy_vec: Vec<_> = enemy_query
            .iter()
            .map(|(e, p)| (e, (player_position.val - p.val).length()))
            .filter(|(_, d)| *d <= player_weapons.range)
            .collect();
        enemy_vec.sort_by(|(_, d_1), (_, d_2)| d_1.partial_cmp(d_2).unwrap());
        for (enemy_entity, _) in enemy_vec.iter().take(shot_count as usize) {
            commands.entity(*enemy_entity).despawn();
        }
    }
}
