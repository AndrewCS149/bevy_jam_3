use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Stamina {
    pub max: f32,
    pub current: f32,
    pub regen_time: Timer,
}

impl Stamina {
    pub fn new(max: f32) -> Self {
        Self {
            max,
            current: max,
            regen_time: Timer::new(Duration::from_secs(2), TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct IsSprinting(pub bool);
