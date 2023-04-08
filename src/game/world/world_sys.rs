use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::game::game_cmps::Game;

use super::{
    world_res::{Colors, LightTimer},
    MAP_SIZE,
};

pub fn spawn_ground(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let _textures = assets.load_folder("textures").unwrap();
    // let floor = assets.get_handle("textures/checkers03_s.jpg");

    let floor = assets.load("textures/checkers03_s.png");

    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: MAP_SIZE,
                ..default()
            })),
            // material: materials.add(Color::GRAY.into()),
            material: floor.clone(),
            ..default()
        },
        Game,
        Collider::cuboid(MAP_SIZE / 2.0, 0.0, MAP_SIZE / 2.0),
        Name::new("Ground"),
    ));
}

pub fn spawn_light(mut cmds: Commands) {
    cmds.spawn((
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                color: Color::GREEN.into(),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Game,
        Name::new("Point Light"),
    ));
}

pub fn change_light_clr(
    mut light_q: Query<&mut PointLight, With<PointLight>>,
    mut light_timer: ResMut<LightTimer>,
    time: Res<Time>,
    colors: Res<Colors>,
) {
    light_timer.0.tick(time.delta());
    if let Ok(mut light) = light_q.get_single_mut() {
        if light_timer.0.finished() {
            let rng = rand::thread_rng().gen_range(0..colors.0.len());
            light.color = colors.0[rng].into();
        }
    }
}
