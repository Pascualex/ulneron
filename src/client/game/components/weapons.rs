use bevy::prelude::*;

#[derive(Component)]
pub struct Weapons {
    pub timer: Timer,
    pub range: f32,
}

impl Weapons {
    pub fn from_hertz(frequency: f32, range: f32) -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / frequency, true),
            range,
        }
    }
}
