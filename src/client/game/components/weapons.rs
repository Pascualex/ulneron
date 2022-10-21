use bevy::prelude::*;

#[derive(Component)]
pub struct Weapons {
    pub damage: u32,
    pub range: f32,
    pub timer: Timer,
}

impl Weapons {
    pub fn from_hertz(damage: u32, frequency: f32, range: f32) -> Self {
        Self {
            damage,
            timer: Timer::from_seconds(1.0 / frequency, TimerMode::Repeating),
            range,
        }
    }
}
