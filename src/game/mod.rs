use bevy::prelude::*;

pub mod camera;
pub mod enemy;
pub mod game_cmps;
pub mod game_res;
mod game_sys;
pub mod hud;
pub mod player;
pub mod powerups;
pub mod projectile;
pub mod world;

use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use game_res::*;
use game_sys::*;
use hud::HudPlugin;
use player::PlayerPlugin;
use powerups::PowerUpsPlugin;
use projectile::ProjectilePlugin;
use world::WorldPlugin;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTime>()
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(WorldPlugin)
            .add_plugin(PowerUpsPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ProjectilePlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(HudPlugin)
            .add_system(exit_game.in_set(OnUpdate(AppState::Game)))
            .add_system(despawn_game.in_schedule(OnExit(AppState::Game)))
            .add_system(tick_game_time.in_set(OnUpdate(AppState::Game)));
    }
}
