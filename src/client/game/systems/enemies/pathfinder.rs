use bevy::prelude::*;

use crate::client::game::{
    components::{Agent, Enemy, Player, Position},
    resources::Ticks,
};

pub fn enemies_pathfinder(
    mut enemy_query: Query<(&Position, &mut Agent), With<Enemy>>,
    player_query: Query<&Position, With<Player>>,
    ticks: Res<Ticks>,
) {
    if ticks.current.is_none() {
        return;
    }

    for (enemy_position, mut enemy_velocity) in enemy_query.iter_mut() {
        let direction = player_query
            .iter()
            .map(|p| p.val - enemy_position.val)
            .min_by(|d1, d2| d1.length().partial_cmp(&d2.length()).unwrap())
            .unwrap_or(Vec2::ZERO);
        enemy_velocity.preferred_velocity = direction.normalize_or_zero() * 3.0;
    }
}
