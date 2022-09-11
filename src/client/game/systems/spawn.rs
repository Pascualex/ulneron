use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    client::game::{
        components::{Enemy, Position, Velocity},
        resources::{Random, Spawner, Ticks},
    },
    TICK_STEP,
};

pub fn spawn(
    mut spawner: ResMut<Spawner>,
    mut random: ResMut<Random>,
    ticks: Res<Ticks>,
    mut commands: Commands,
) {
    if ticks.current.is_none() {
        return;
    }

    spawner.timer.tick(Duration::from_secs_f32(TICK_STEP));
    for _ in 0..spawner.timer.times_finished_this_tick() {
        let x = random.gen_range(-5.0..=5.0);
        let y = random.gen_range(-5.0..=5.0);
        commands
            .spawn()
            .insert(Position::from_xy(x, y))
            .insert(Velocity::from_xy(0.0, 0.0))
            .insert(Enemy);
    }
}
