use std::time::Duration;

use bevy::prelude::*;

use crate::{
    client::game::{
        components::{Enemy, Health, Player, Position, Velocity, Weapons},
        resources::Ticks,
    },
    TICK_STEP,
};

pub fn players_attack(
    mut player_query: Query<(&Position, &Velocity, &mut Weapons), With<Player>>,
    mut enemy_query: Query<(&Position, &mut Health), With<Enemy>>,
    ticks: Res<Ticks>,
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
            .iter_mut()
            .map(|(p, h)| ((player_position.val - p.val).length(), h))
            .filter(|(d, _)| *d <= player_weapons.range)
            .collect();
        enemy_vec.sort_by(|(d_1, _), (d_2, _)| d_1.partial_cmp(d_2).unwrap());
        for _ in 0..shot_count {
            for (_, enemy_health) in enemy_vec.iter_mut() {
                if !enemy_health.dead() {
                    enemy_health.damage(player_weapons.damage);
                    break;
                }
            }
        }
    }
}
