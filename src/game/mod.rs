use bevy::prelude::*;

pub mod camera;
pub mod enemy;
pub mod game_cmps;
pub mod gamepad;
pub mod hud;
pub mod player;
pub mod powerups;
pub mod projectile;
pub mod world;

use camera::CameraPlugin;
use enemy::EnemyPlugin;
use gamepad::GamepadPlugin;
use hud::HudPlugin;
use player::PlayerPlugin;
use powerups::PowerUpsPlugin;
use projectile::ProjectilePlugin;
use world::WorldPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldPlugin)
            .add_plugin(PowerUpsPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ProjectilePlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(GamepadPlugin)
            .add_plugin(HudPlugin);
    }
}
