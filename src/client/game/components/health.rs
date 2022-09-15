use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub health: u32,
    pub max_health: u32,
}

impl Health {
    pub fn new(max_health: u32) -> Self {
        Self {
            health: max_health,
            max_health,
        }
    }

    pub fn damage(&mut self, amount: u32) {
        self.health = self.health.saturating_sub(amount);
    }

    pub fn heal(&mut self, amount: u32) {
        self.health = u32::max(self.health + amount, self.max_health);
    }

    pub fn dead(&self) -> bool {
        self.health == 0
    }
}
