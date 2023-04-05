use std::time::Duration;

use bevy::prelude::*;

use super::*;

#[derive(Resource)]
pub struct PowerUpSpawnTime(pub Timer);

impl Default for PowerUpSpawnTime {
    fn default() -> Self {
        PowerUpSpawnTime(Timer::from_seconds(
            POWERUP_SPAWN_TIME,
            TimerMode::Repeating,
        ))
    }
}

#[derive(Resource)]
pub struct DamageDuration(pub Timer);

impl Default for DamageDuration {
    fn default() -> Self {
        let mut duration = Timer::new(Duration::from_secs(DMG_BOOST_DURATION), TimerMode::Once);
        duration.pause();
        DamageDuration(duration)
    }
}
