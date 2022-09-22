use std::time::Duration;

use bevy::prelude::*;
use kiddo::distance::squared_euclidean;

use crate::{
    client::game::{
        components::{Enemy, Health, Player, Position, Velocity, Weapons},
        resources::{SpacePartitioner, Ticks},
    },
    TICK_STEP,
};

pub fn players_attack(
    mut player_query: Query<(&Position, &Velocity, &mut Weapons), With<Player>>,
    mut enemy_query: Query<(&Position, &mut Health), With<Enemy>>,
    space_partitioner: Res<SpacePartitioner>,
    ticks: Res<Ticks>,
) {
    if ticks.current.is_none() {
        return;
    }

    for (player_position, player_velocity, mut player_weapons) in player_query.iter_mut() {
        if player_velocity.val != Vec2::ZERO {
            continue;
        }
        let tick_duration = Duration::from_secs_f32(TICK_STEP);
        player_weapons.timer.tick(tick_duration);
        let mut shot_count = player_weapons.timer.times_finished_this_tick();
        let squared_range = player_weapons.range.powi(2);
        for (squared_distance, enemy_entity) in space_partitioner
            .enemies
            .iter_nearest(player_position.val.as_ref(), &squared_euclidean)
            .unwrap()
        {
            if squared_distance > squared_range {
                break;
            }
            let (_, mut enemy_health) = enemy_query.get_mut(*enemy_entity).unwrap();
            while !enemy_health.dead() && shot_count > 0 {
                enemy_health.damage(player_weapons.damage);
                shot_count -= 1;
            }
            if shot_count == 0 {
                break;
            }
        }
    }
}
