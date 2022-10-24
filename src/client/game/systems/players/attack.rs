use std::time::Duration;

use bevy::prelude::*;
use kiddo::distance::squared_euclidean;

use crate::{
    client::game::{
        components::{Agent, Enemy, Health, Player, Position, Resources, Weapons},
        resources::{SpacePartitioner, Ticks},
    },
    TICK_STEP,
};

pub fn players_attack(
    mut player_query: Query<(&Position, &Agent, &mut Weapons, &mut Resources), With<Player>>,
    mut enemy_query: Query<&mut Health, With<Enemy>>,
    space_partitioner: Res<SpacePartitioner>,
    ticks: Res<Ticks>,
) {
    if ticks.current.is_none() {
        return;
    }

    for (position, agent, mut weapons, mut resources) in player_query.iter_mut() {
        if agent.preferred_velocity != Vec2::ZERO {
            continue;
        }
        let tick_duration = Duration::from_secs_f32(TICK_STEP);
        weapons.timer.tick(tick_duration);
        let mut shot_count = weapons.timer.times_finished_this_tick();
        let squared_range = weapons.range.powi(2);
        for (squared_distance, enemy_entity) in space_partitioner
            .enemies
            .iter_nearest(position.val.as_ref(), &squared_euclidean)
            .unwrap()
        {
            if squared_distance > squared_range {
                break;
            }
            let mut enemy_health = enemy_query.get_mut(*enemy_entity).unwrap();
            if enemy_health.dead() {
                continue;
            }
            while shot_count > 0 {
                enemy_health.damage(weapons.damage);
                shot_count -= 1;
                if enemy_health.dead() {
                    resources.nerite += 7;
                    resources.kills += 1;
                    break;
                }
            }
            if shot_count == 0 {
                break;
            }
        }
    }
}
