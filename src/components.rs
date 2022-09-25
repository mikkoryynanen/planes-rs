use bevy::prelude::Component;

#[derive(Component)]
pub struct Health {
    pub amount: i32,
}

impl Health {
    pub fn take_damage(&mut self, amount: i32) {
        self.amount -= amount;
    }
}

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Collectable;
