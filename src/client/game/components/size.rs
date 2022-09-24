use bevy::prelude::Component;

#[derive(Component)]
pub struct Size {
    pub radius: f32,
}

impl Size {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}
